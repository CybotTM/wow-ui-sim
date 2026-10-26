import importlib.util
import struct
import tempfile
import unittest
from pathlib import Path

SPEC = importlib.util.spec_from_file_location(
    "mists_strings", Path(__file__).resolve().parents[1] / "gen_mists_global_strings.py"
)
GEN = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(GEN)


def two_section_db2():
    """A decrypted WDC5 fixture: strings use concatenated logical spaces."""
    strings = [b'FIRST\x00Line one\n"quoted" %d\x00', b'SECOND\x00Other %s\x00']
    record_bytes = 24
    palette_offset = 368
    first_offset = 404
    second_offset = first_offset + 12 + len(strings[0]) + 4
    data = bytearray(second_offset + 12 + len(strings[1]) + 4)
    data[:4] = b'WDC5'
    struct.pack_into('<I', data, 4, 5)
    struct.pack_into('<9I2H7I', data, 136,
                     2, 3, 12, sum(map(len, strings)), 0, 0, 1, 2, 1,
                     4, 0, 3, 8, 0, 72, 0, 28, 2)
    struct.pack_into('<HHIIIII', data, 296, 0, 32, 0, 0, 0, 0, 0)
    struct.pack_into('<HHIIIII', data, 320, 32, 32, 0, 0, 0, 0, 0)
    struct.pack_into('<HHIIIII', data, 344, 64, 3, 28, 3, 0, 3, 0)
    struct.pack_into('<7I', data, palette_offset, 1, 3, 7, 2, 5, 0, 4)
    for index, offset in enumerate([first_offset, second_offset]):
        encrypted_key = 0 if index == 0 else 0x409FC8B6C2286508
        struct.pack_into('<Q8I', data, 204 + index * 40,
                         encrypted_key, offset, 1, len(strings[index]), 0, 4, 0, 0, 0)
        logical_string_offset = 0 if index == 0 else len(strings[0])
        relative_tag = record_bytes + logical_string_offset - index * 12
        relative_text = relative_tag + strings[index].index(b'\x00') + 1 - 4
        struct.pack_into('<iiI', data, offset, relative_tag, relative_text, index)
        data[offset + 12:offset + 12 + len(strings[index])] = strings[index]
        struct.pack_into('<I', data, offset + 12 + len(strings[index]), index + 1)
    return bytes(data)


class MistsStringGeneratorTests(unittest.TestCase):
    def test_decrypted_second_section_and_multiline_csv_round_trip(self):
        rows = GEN.decode_db2(two_section_db2())
        self.assertEqual(rows, [(1, 'FIRST', 'Line one\n"quoted" %d', 1),
                                (2, 'SECOND', 'Other %s', 3)])
        with tempfile.TemporaryDirectory() as directory:
            source = Path(directory) / 'input.csv'
            output = Path(directory) / 'output.rs'
            GEN.write_csv(rows, source)
            GEN.generate(source, output)
            self.assertIn('"FIRST" => "Line one\\n\\"quoted\\" %d",', output.read_text())
            self.assertIn('"SECOND" => "Other %s",', output.read_text())

    def test_invalid_string_offset_rejected(self):
        data = bytearray(two_section_db2())
        struct.pack_into('<i', data, 404, -100)
        with self.assertRaisesRegex(ValueError, 'string offset'):
            GEN.decode_db2(bytes(data))

    def test_duplicate_tag_does_not_replace_output(self):
        with tempfile.TemporaryDirectory() as directory:
            source = Path(directory) / 'input.csv'
            output = Path(directory) / 'output.rs'
            source.write_text('ID,BaseTag,TagText_lang,Flags\n1,A,first,1\n2,A,second,1\n')
            output.write_text('unchanged')
            with self.assertRaisesRegex(ValueError, 'duplicate tag'):
                GEN.generate(source, output)
            self.assertEqual(output.read_text(), 'unchanged')


if __name__ == '__main__':
    unittest.main()
