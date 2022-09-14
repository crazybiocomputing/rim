/*
 * Series of tests for mm_star parser
 * Authors:
 * Jean-Christophe Taveau
 *
 */

const file00: String = "data_BIOINFORMATICS";

const file01: String = "# This is a comment

data_1ZNI
";

const file02: String = "# No data_ keyword

_header.id       Test
";

const file03: String = "data_1ZNI
#
_header.id       Test
";

const file04: String = "data_1ZNI
#
_header.version  1.2.3
";

const file05: String = "data_1ZNI
#
_header.title    'Unitary Tests'
";

const file06: String = "data_1ZNI
#
_header.text
;This is a multiline String

Lorem ipsum dolor sit amet, consectetur adipiscing elit. 
Pellentesque imperdiet vestibulum justo, a porttitor diam 
scelerisque sit amet. Aliquam ligula sapien, viverra vel 
finibus ut, semper non neque.
;
#
";

const file07: String = "data_1ZNI
#
_header.number   1.2345678
#
";

const file08: String = "data_1ZNI
#
loop_
_data.id
_data.value
_data.name
1    10    X
2    20    Y
3    30    Z
#
";

const file09: String = "data_MUSIC
#
loop_
_data.id
_data.value
_data.name
1    10    'The Stones'
2    20    'Deep Purple'
3    30    Stromae
#
";

const file10: String = "data_MUSIC
#
loop_
_data.id
_data.value
_data.name
_data.title
_data.year
1    10    'The Stones'
'Sympathy for the Devil. Beggars Banquet' 1968
2    20    'Deep Purple'
'Smoke on the Water. Machine Head' 1972
3    30    Stromae Papaoutai ?
#
";

const file11: String = "data_MUSIC
#
loop_
_data.id
_data.members
_data.name
_data.title
_data.lyrics
_data.year
1    4    'The Stones' 'Sympathy for the Devil. Beggars Banquet'
;Please allow me to introduce myself
I'm a man of wealth and taste
I've been around for a long, long years
;
1968
2    5    'Deep Purple' 'Smoke on the Water. Machine Head'
;Smoke on the water
A fire in the sky
Smoke on the water
;
1972
3    1    Stromae Papaoutai
;Sans même devoir lui parler
Il sait ce qu'il ne va pas
Un sacré papa
;
?
#
";

//////////////////// UNITARY TESTS FOR STAR READER /////////////////////////

#[test]
fn should_contain_datablock_equal_to_bioinformatics() {
    let molecule = parse_star(file00);
    assert_eq!(molecule.datablock, "BIOINFORMATICS");
}

/*
    #[test]
    fn test_bad_add() {
        // This assert would fire and test will fail.
        // Please note, that private functions can be tested too!
        assert_eq!(bad_add(1, 2), 3);
    }

    #[test]
    it('should not throw an Error of unknown format', function () {
      let boom = () => parse_star(file01);
      expect(boom).not.toThrow();
    });

    #[test]
    fn should_contain_datablock_equal_to_1ZNI () {
      let molecule = parse_star(file01);
        let molecule = parse_star(file00);
        assert_eq!(molecule.datablock,"1ZNI");
    });
  });

  describe('File#02', function () {

    it('should throw an Error Message', function () {
      const boom = () => parse_star(file02);
      expect(boom).toThrow();
    });
  });


  describe('File#03', function () {
    let molecule;
    beforeEach(function() {
      molecule = parse_star(file03);
    });

    fn should contain "datablock" and category/attribute', function () {
      expect(molecule.datablock).toBeDefined();
      expect(Object.keys(molecule)).toContain("header");
      expect(molecule.header).toBeDefined();
      expect(Object.keys(molecule.header)).toContain("id");

    });

    it('should have "1ZNI"in datablock and "Test" in header.id', function () {
      expect(molecule.datablock).toContain("1ZNI");
      expect(molecule.header.id).toBe("Test");
    });
  });

  describe('File#04', function () {
    let molecule;
    beforeEach(function() {
      molecule = parse_star(file04);
    });

    it('should have "1ZNI"in datablock and "1.2.3" in header.version', function () {
      expect(molecule.datablock).toBeDefined();
      expect(Object.keys(molecule)).toContain("header");
      expect(molecule.header).toBeDefined();
      expect(Object.keys(molecule.header)).toContain("version");
      expect(molecule.datablock).toContain("1ZNI");
      expect(molecule.header.version).toBe("1.2.3");
    });
  });

  describe('File#05', function () {
    let molecule;
    beforeEach(function() {
      molecule = parse_star(file05);
    });

    it('should have "1ZNI"in datablock and "Unitary Tests" in header.title', function () {
      expect(molecule.datablock).toBeDefined();
      expect(Object.keys(molecule)).toContain("header");
      expect(molecule.header).toBeDefined();
      expect(Object.keys(molecule.header)).toContain("title");
      expect(molecule.datablock).toContain("1ZNI");
      expect(molecule.header.title).toBe("Unitary Tests");
    });
  });

  describe('File#06', function () {
    let molecule;
    beforeEach(function() {
      molecule = parse_star(file06);
    });

    it('should have a datablock and multiline String in header.text', function () {
      expect(molecule.datablock).toBeDefined();
      expect(Object.keys(molecule)).toContain("header");
      expect(molecule.header).toBeDefined();
      expect(Object.keys(molecule.header)).toContain("text");
    });

    it('should have "1ZNI" in datablock', function () {
      expect(molecule.datablock).toContain("1ZNI");
    });
    it('should have a multiline String of 232 of length', function () {
      const txt = "This is a multiline String

Lorem ipsum dolor sit amet, consectetur adipiscing elit.
Pellentesque imperdiet vestibulum justo, a porttitor diam
scelerisque sit amet. Aliquam ligula sapien, viverra vel
finibus ut, semper non neque.\n";

      expect(molecule.header.text).toContain("Aliquam ligula sapien");
      expect(molecule.header.text.length).toBe(txt.length);
    });
  });


  describe('File#07', function () {
    let molecule;
    beforeEach(function() {
      molecule = parse_star(file07);
    });

    it('should have "1ZNI"in datablock and a floating point number in header.number', function () {
      expect(molecule.datablock).toBeDefined();
      expect(Object.keys(molecule)).toContain("header");
      expect(molecule.header).toBeDefined();
      expect(Object.keys(molecule.header)).toContain("number");
      expect(molecule.header.number).toBe(1.2345678);
    });
  });

  describe('File#08', function () {
    let molecule;
    beforeEach(function() {
      molecule = parse_star(file08);
    });

    it('should have tabular data with header and rows', function () {
      expect(molecule.datablock).toBeDefined();
      expect(Object.keys(molecule)).toContain("data");
      expect(molecule.data).toBeDefined();
      expect(Object.keys(molecule.data)).toContain("header");
      expect(Object.keys(molecule.data)).toContain("rows");
    });

    it('should have a header of 3 items: id, value, and name', function () {
      expect(molecule.data.header.length).toBe(3);
      expect(molecule.data.header).toEqual(["id","value","name"]);
    });

    it('should have three rows containing each 3 items', function () {
      expect(molecule.data.rows.length).toBe(3);
      expect(molecule.data.rows[0].length).toBe(3);
      expect(molecule.data.rows[1].length).toBe(3);
      expect(molecule.data.rows[2].length).toBe(3);
      expect(molecule.data.rows[0]).toEqual([1, 10, 'X']);
      expect(molecule.data.rows[1]).toEqual([2, 20, 'Y']);
      expect(molecule.data.rows[2]).toEqual([3, 30, 'Z']);

    });
  });

  describe('File#09', function () {
    let molecule;
    beforeEach(function() {
      molecule = parse_star(file09);
    });

    it('should have tabular data with header and rows', function () {
      expect(molecule.datablock).toBeDefined();
      expect(Object.keys(molecule)).toContain("data");
      expect(molecule.data).toBeDefined();
      expect(Object.keys(molecule.data)).toContain("header");
      expect(Object.keys(molecule.data)).toContain("rows");
    });

    it('should have a header of 3 items: id, value, and name', function () {
      expect(molecule.data.header.length).toBe(3);
      expect(molecule.data.header).toEqual(["id","value","name"]);
    });
    it('should have three rows containing each 3 items', function () {
      expect(molecule.data.rows.length).toBe(3);
      expect(molecule.data.rows[0].length).toBe(3);
      expect(molecule.data.rows[1].length).toBe(3);
      expect(molecule.data.rows[2].length).toBe(3);
    });
    it('should have first row containing [1, 10, "The Stones"]', function () {
      expect(molecule.data.rows[0]).toEqual([1, 10, 'The Stones']);
    });
    it('should have second row containing [2, 20, "Deep Purple"]', function () {
      expect(molecule.data.rows[1]).toEqual([2, 20, 'Deep Purple']);
    });
    it('should have third row containing [3, 30, "Stromae"]', function () {
      expect(molecule.data.rows[2]).toEqual([3, 30, 'Stromae']);
    });
  });


  describe('File#10', function () {
    let molecule;
    beforeEach(function() {
      molecule = parse_star(file10);
    });

    it('should have tabular data with header and rows', function () {
      expect(molecule.datablock).toBeDefined();
      expect(Object.keys(molecule)).toContain("data");
      expect(molecule.data).toBeDefined();
      expect(Object.keys(molecule.data)).toContain("header");
      expect(Object.keys(molecule.data)).toContain("rows");
    });

    it('should have a header of 5 items: id, value, name, title, and year', function () {
      expect(molecule.data.header.length).toBe(5);
      expect(molecule.data.header).toEqual(["id","value","name","title","year"]);
    });
    it('should have three rows containing each 5 items', function () {
      expect(molecule.data.rows.length).toBe(3);
      expect(molecule.data.rows[0].length).toBe(5);
      expect(molecule.data.rows[1].length).toBe(5);
      expect(molecule.data.rows[2].length).toBe(5);
    });
    it('should have first row containing [1, 10, "The Stones","Sympathy for the Devil. Beggars Banquet",1968]', function () {
      expect(molecule.data.rows[0]).toEqual([1, 10, 'The Stones','Sympathy for the Devil. Beggars Banquet',1968]);
    });
    it('should have second row containing [2, 20, "Deep Purple","Smoke on the Water. Machine Head", 1972]', function () {
      expect(molecule.data.rows[1]).toEqual([2, 20, 'Deep Purple',"Smoke on the Water. Machine Head", 1972]);
    });
    it('should have third row containing [3, 30, "Stromae","Papaoutai","?"]', function () {
      expect(molecule.data.rows[2]).toEqual([3, 30, 'Stromae',"Papaoutai","?"]);
    });

  });

  describe('File#11', function () {
    let molecule;
    beforeEach(function() {
      molecule = parse_star(file11);
    });

    it('should have tabular data with header and rows', function () {
      expect(molecule.datablock).toBeDefined();
      expect(Object.keys(molecule)).toContain("data");
      expect(molecule.data).toBeDefined();
      expect(Object.keys(molecule.data)).toContain("header");
      expect(Object.keys(molecule.data)).toContain("rows");
    });

    it('should have a header of 5 items: id, members, name, title, lyrics, and year', function () {
      expect(molecule.data.header.length).toBe(6);
      expect(molecule.data.header).toEqual(["id","members","name","title","lyrics","year"]);
    });
    it('should have three rows containing each 5 items', function () {
      expect(molecule.data.rows.length).toBe(3);
      expect(molecule.data.rows[0].length).toBe(6);
      expect(molecule.data.rows[1].length).toBe(6);
      expect(molecule.data.rows[2].length).toBe(6);
    });
    it('should have first row containing [1, 4, "The Stones","Sympathy for the Devil. Beggars Banquet",<lyrics>, 1968]', function () {
      expect(molecule.data.rows[0].slice(0,4)).toEqual([1, 4, 'The Stones','Sympathy for the Devil. Beggars Banquet']);
      expect(molecule.data.rows[0][4]).toContain("Please allow me to introduce myself\nI'm a man of wealth and taste\nI've been around for a long, long years\n");
      expect(molecule.data.rows[0][5]).toBe(1968);
    });
    it('should have second row containing [2, 5, "Deep Purple","Smoke on the Water. Machine Head", <lyrics>, 1972]', function () {
      expect(molecule.data.rows[1].slice(0,4)).toEqual([2, 5, 'Deep Purple',"Smoke on the Water. Machine Head"]);
      expect(molecule.data.rows[1][4]).toBe("Smoke on the water\nA fire in the sky\nSmoke on the water\n");
      expect(molecule.data.rows[1][5]).toBe(1972);
    });
    it('should have third row containing [3, 1, "Stromae","Papaoutai",<lyrics>, "?"]', function () {
      expect(molecule.data.rows[2].slice(0,4)).toEqual([3, 1, 'Stromae',"Papaoutai"]);
      expect(molecule.data.rows[2][4]).toBe("Sans même devoir lui parler\nIl sait ce qu'il ne va pas\nUn sacré papa\n");
      expect(molecule.data.rows[2][5]).toContain('?');
    });

  });

*/
