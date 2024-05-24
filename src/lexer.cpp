#include <string>
#include <vector>

enum class LexemaTYPE {
  // primitive types
  INTEGER,
  TEXT,
  REAL,
  // comments
  COMMENT,
  // parenthesis
  PARENTHESIS_LEFT,
  PARENTHESIS_RIGHT,
  // vectors
  PARENTHESIS_VECTOR_LEFT,
  PARENTHESIS_VECTOR_RIGHT,
  // separator
  SEMICOLON,
  // scope separator
  PARENTHESIS_SCOPE_LEFT,
  PARENTHESIS_SCOPE_RIGHT,
  // key words
  KEY_WORD,
  // definition
  EQUALS,
  // functions
  FUNCTION,
};

enum class KeyWord {
  FN,  // functions
  LET, // let expressions
  DO,  // do notation
};

enum class PrimitiveFunctions {
  // Text operations
  CONCAT,
  // Arithmetic
  ADDITION,
  NEGATION,
  SUBTRACTION,
  MULTIPLICATION,
  // IO
  PRINT,
  GETLINE,
  NEWLINE,
};

struct Lexema {
  LexemaTYPE type;
  std::string value;
};

struct Lexemata {
  std::vector<Lexemata> lexemes;
  size_t location;
};

struct Function {
  std::string name;
  std::vector<Lexemata> parameters;
  Lexemata definition;
};

struct Let {
  std::vector<Lexemata> terms;
};

struct Do {
  std::vector<Lexemata> terms;
};

Lexemata analyze(const std::string *input) {
  Lexemata lexemata;

  return lexemata;
}
