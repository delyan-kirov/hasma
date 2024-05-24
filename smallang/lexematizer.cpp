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

void lexematize(const std::string &source) {
  size_t currentLine = 0;
  size_t position = 0;
  size_t linePosition = 0;

  std::vector<Lexema> lexemes;

  while (position < source.size()) {
    char currentChar = source.at(position);

    switch (currentChar) {
    case ' ':
      break;
    case '\n':
      ++currentLine;
      linePosition = 0;
      break;
    case ';':
      lexemes.push_back(Lexema{LexemaType::DIVIDOR, ";",
                               Position{currentLine, linePosition}});
      break;
    case '/':
      if (source.at(position + 1) == '/') {
        ++position;
        ++linePosition;
        while (source.at(position) != '\n') {
          ++position;
          ++linePosition;
        }
      } else {
        std::cerr
            << "\nERROR: the char / is not recognized outside of // for now\n";
        exit(1);
      }
      break;
    case '=':
      lexemes.push_back(
          Lexema{LexemaType::EQUAL, "=", Position{currentLine, linePosition}});
      break;
    case '(':
      lexemes.push_back(Lexema{LexemaType::PAREN_LEFT, "(",
                               Position{currentLine, linePosition}});
      break;
    case ')':
      lexemes.push_back(Lexema{LexemaType::PAREN_RIGHT, ")",
                               Position{currentLine, linePosition}});
      break;
    case '"': {
      std::string text(1, currentChar);
      ++position;
      ++linePosition;

      while (source.at(position) != '"') {
        text += source.at(position);
        ++position;
        ++linePosition;
        if (position > source.size()) {
          std::cerr << "ERROR: Did not find the closing \"\n";
          exit(1);
        }
      }

      if (position < source.size()) {
        text += '"';
        ++position;
        ++linePosition;
      }

      lexemes.push_back(
          Lexema{LexemaType::TEXT, text, Position{currentLine, linePosition}});
      break;
    }
    default:
      if (isdigit(source.at(position))) {
        std::string number;

        while (position < source.size() && isdigit(source.at(position))) {
          number += source.at(position);
          ++position;
        }

        lexemes.push_back(Lexema{LexemaType::NUMBER, number,
                                 Position{currentLine, linePosition}});
        --position;
      } else if (isalpha(source.at(position))) {
        std::string variable(1, currentChar);
        ++position;
        if (source.at(position) == ')') {
          --position;
          lexemes.push_back(Lexema{LexemaType::ALIAS, variable,
                                   Position{currentLine, linePosition}});
        } else {

          while (position < source.size() &&
                 (isalnum(source.at(position)) || source.at(position) == '_')) {

            variable += source.at(position);
            ++position;
            ++linePosition;
          }

          lexemes.push_back(Lexema{LexemaType::ALIAS, variable,
                                   Position{currentLine, linePosition}});
        }
      } else {
        std::cerr << "\nERROR: Did not recognize lexeme: " << currentChar
                  << "\n";
        exit(1);
      }
      break;
    }

    ++position;
    ++linePosition;
  }

  // For demonstration, print the collected lexemes
  for (const auto &lex : lexemes) {
    std::cout << "\nLexeme: " << lex.lexeme << " at line " << lex.position.line
              << ", column " << lex.position.column;
  }
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
