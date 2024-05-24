#include <cctype>
#include <cstddef>
#include <cstdlib>
#include <fstream>
#include <iostream>
#include <sstream>
#include <string>
#include <vector>

enum class LexemaType {
  PAREN_LEFT,
  PAREN_RIGHT,
  ALIAS,
  DIVIDOR,
  EQUAL,
  NUMBER,
  TEXT,
};

struct Position {
  size_t line;
  size_t column;
};

struct Lexema {
  LexemaType type;
  std::string lexeme;
  Position position;
};

void skipWhitespace(const std::string &source, size_t &position) {
  while (position < source.size() && std::isspace(source[position])) {
    ++position;
  }
}

void handleEndOfLine(size_t &currentLine, size_t &linePosition) {
  ++currentLine;
  linePosition = 0;
}

void processText(const std::string &source, size_t &position,
                 size_t &linePosition, size_t currentLine,
                 std::vector<Lexema> &lexemes) {
  std::string text;
  ++position;
  ++linePosition;

  while (position < source.size() && source[position] != '"') {
    text += source[position];
    ++position;
    ++linePosition;
  }

  if (position >= source.size() || source[position] != '"') {
    std::cerr << "ERROR: Did not find the closing \"\n";
    exit(1);
  }

  text += '"';
  ++position;
  ++linePosition;

  lexemes.push_back(
      Lexema{LexemaType::TEXT, text, Position{currentLine, linePosition}});
}

void processNumber(const std::string &source, size_t &position,
                   size_t &linePosition, size_t currentLine,
                   std::vector<Lexema> &lexemes) {
  std::string number;

  while (position < source.size() && std::isdigit(source[position])) {
    number += source[position];
    ++position;
    ++linePosition;
  }

  lexemes.push_back(
      Lexema{LexemaType::NUMBER, number, Position{currentLine, linePosition}});
}

void processVariable(const std::string &source, size_t &position,
                     size_t &linePosition, size_t currentLine,
                     std::vector<Lexema> &lexemes) {
  std::string variable;
  variable += source[position];
  ++position;
  ++linePosition;

  while (position < source.size() &&
         (std::isalnum(source[position]) || source[position] == '_')) {
    variable += source[position];
    ++position;
    ++linePosition;
  }

  lexemes.push_back(
      Lexema{LexemaType::ALIAS, variable, Position{currentLine, linePosition}});
}

void handleError(char currentChar) {
  std::cerr << "\nERROR: Did not recognize lexeme: " << currentChar << "\n";
  exit(1);
}

void printLexemes(std::vector<Lexema> &lexemes) {
  // For demonstration, print the collected lexemes
  for (const auto &lex : lexemes) {
    std::cout << "\nLexeme: " << lex.lexeme << " at line " << lex.position.line
              << ", column " << lex.position.column;
  }
}

std::vector<Lexema> lexematize(const std::string &source) {
  size_t currentLine = 0;
  size_t position = 0;
  size_t linePosition = 0;

  std::vector<Lexema> lexemes;

  while (position < source.size() - 1) {
    skipWhitespace(source, position);

    char currentChar = source.at(position);

    switch (currentChar) {
    case '\n':
      handleEndOfLine(currentLine, linePosition);
      break;
    case ';':
      lexemes.push_back(
          Lexema{LexemaType::DIVIDOR, ";", {currentLine, linePosition}});
      break;
    case '/':
      if (source.at(position + 1) == '/') {
        ++position;
        while (source.at(position) != '\n') {
          ++position;
        }
        linePosition = 0;
      } else {
        std::cerr
            << "\nERROR: the char / is not recognized outside of // for now\n";
        exit(1);
      }
      break;
    case '=':
      lexemes.push_back(
          Lexema{LexemaType::EQUAL, "=", {currentLine, linePosition}});
      break;
    case '(':
      lexemes.push_back(
          Lexema{LexemaType::PAREN_LEFT, "(", {currentLine, linePosition}});
      break;
    case ')':
      lexemes.push_back(
          Lexema{LexemaType::PAREN_RIGHT, ")", {currentLine, linePosition}});
      break;
    case '"':
      processText(source, position, linePosition, currentLine, lexemes);
      break;
    default:
      if (isdigit(currentChar)) {
        processNumber(source, position, linePosition, currentLine, lexemes);
      } else if (isalpha(currentChar)) {
        processVariable(source, position, linePosition, currentLine, lexemes);
      } else {
        handleError(currentChar);
      }
      break;
    }

    ++position;
    ++linePosition;
  }

  printLexemes(lexemes);

  return lexemes;
}

const static std::string source = "a = 2\n"
                                  ";\n"
                                  "\n"
                                  "b = times a 2\n"
                                  ";\n";

std::string readFile(const std::string &filePath) {
  std::ifstream file(filePath);
  if (!file.is_open()) {
    throw std::runtime_error("Could not open file");
  }

  std::stringstream buffer;
  buffer << file.rdbuf();
  return buffer.str();
}

int main() {
  lexematize(source);

  const std::string filePath = "./example.smallang";

  std::cout << "\n\nINFO: Tokenizing example\n";

  try {
    std::string fileContent = readFile(filePath);
    lexematize(fileContent);
  } catch (const std::exception &e) {
    std::cerr << e.what() << std::endl;
    return 1;
  }
  return 0;
}

// TODO Fix ;
// TODO Fix )
