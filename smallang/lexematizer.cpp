#include <cctype>
#include <cstddef>
#include <cstdlib>
#include <fstream>
#include <iostream>
#include <memory>
#include <sstream>
#include <string>
#include <variant>
#include <vector>

enum class LexemaType
{
  PAREN_LEFT,
  PAREN_RIGHT,
  ALIAS,
  DIVIDOR,
  EQUAL,
  NUMBER,
  TEXT,
};

std::string
LexemaTypeToString(LexemaType type)
{
  switch (type) {
    case LexemaType::PAREN_LEFT:
      return "PAREN_LEFT";
    case LexemaType::PAREN_RIGHT:
      return "PAREN_RIGHT";
    case LexemaType::ALIAS:
      return "ALIAS";
    case LexemaType::DIVIDOR:
      return "DIVIDOR";
    case LexemaType::EQUAL:
      return "EQUAL";
    case LexemaType::NUMBER:
      return "NUMBER";
    case LexemaType::TEXT:
      return "TEXT";
    default:
      return "UNKNOWN";
  }
}

struct Position
{
  size_t line;
  size_t column;
};

struct Lexema
{
  LexemaType type;
  std::string lexeme;
  Position position;
};

void
skipWhitespace(const std::string* source,
               size_t& position,
               size_t& linePosition)
{
  while (position < (*source).size() && std::isspace((*source)[position])) {
    ++position;
    ++linePosition;
  }
}

void
handleEndOfLine(size_t& currentLine, size_t& linePosition)
{
  ++currentLine;
  linePosition = 0;
}

void
processText(const std::string* source,
            size_t& position,
            size_t& linePosition,
            size_t currentLine,
            std::vector<Lexema>* lexemes)
{
  std::string text;
  ++position;
  ++linePosition;

  while (position < (*source).size() && source->at(position) != '"') {
    text += (*source).at(position);
    ++position;
    ++linePosition;
  }

  if (position >= (*source).size() || source->at(position) != '"') {
    std::cerr << "ERROR: Did not find the closing \"\n";
    exit(1);
  }

  text += '"';
  ++position;
  ++linePosition;

  lexemes->push_back(
    Lexema{ LexemaType::TEXT, text, Position{ currentLine, linePosition } });
}

void
processNumber(const std::string* source,
              size_t& position,
              size_t& linePosition,
              size_t& currentLine,
              std::vector<Lexema>* lexemes)
{
  std::string number;
  number += (*source).at(position);

  while (position < source->size()) {
    if (std::isdigit(source->at(position + 1))) {
      number += (source->at(position));
      ++position;
      ++linePosition;
    } else {
      lexemes->push_back(Lexema{
        LexemaType::NUMBER, number, Position{ currentLine, linePosition } });
      return;
    }
  }
}

void
processVariable(const std::string* source,
                size_t& position,
                size_t& linePosition,
                size_t& currentLine,
                std::vector<Lexema>* lexemes)
{
  std::string variable;
  variable += source->at(position);

  while (position < source->size()) {
    if ((std::isalnum(source->at(position + 1)) ||
         source->at(position + 1) == '_')) {
      variable += source->at(position + 1);
      ++position;
      ++linePosition;
    } else {
      lexemes->push_back(Lexema{
        LexemaType::ALIAS, variable, Position{ currentLine, linePosition } });
      return;
    }
  }
}

void
handleError(char currentChar)
{
  std::cerr << "\nERROR: Did not recognize lexeme: " << currentChar << "\n";
  exit(1);
}

void
printLexemes(std::vector<Lexema>* lexemes)
{
  // For demonstration, print the collected lexemes
  for (const auto& lex : *lexemes) {
    std::cout << "Lexeme: " << lex.lexeme << " at line " << lex.position.line
              << ", column " << lex.position.column << ", type "
              << LexemaTypeToString(lex.type) << "\n";
  }
}

std::unique_ptr<std::vector<Lexema>>
lexematize(const std::string* source)
{
  size_t currentLine = 0;
  size_t position = 0;
  size_t linePosition = 0;

  auto lexemes = std::make_unique<std::vector<Lexema>>();

  while (position < source->size() - 1) {
    skipWhitespace(source, position, linePosition);

    char currentChar = source->at(position);

    switch (currentChar) {
      case '\n':
        handleEndOfLine(currentLine, linePosition);
        break;
      case ';':
        lexemes->push_back(
          Lexema{ LexemaType::DIVIDOR, ";", { currentLine, linePosition } });
        break;
      case '/':
        if (source->at(position + 1) == '/') {
          ++position;
          while (source->at(position) != '\n') {
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
        lexemes->push_back(
          Lexema{ LexemaType::EQUAL, "=", { currentLine, linePosition } });
        break;
      case '(':
        lexemes->push_back(
          Lexema{ LexemaType::PAREN_LEFT, "(", { currentLine, linePosition } });
        break;
      case ')':
        lexemes->push_back(Lexema{
          LexemaType::PAREN_RIGHT, ")", { currentLine, linePosition } });
        break;
      case '"':
        processText(source, position, linePosition, currentLine, lexemes.get());
        break;
      default:
        if (isdigit(currentChar)) {
          processNumber(
            source, position, linePosition, currentLine, lexemes.get());
        } else if (isalpha(currentChar)) {
          processVariable(
            source, position, linePosition, currentLine, lexemes.get());
        } else {
          handleError(currentChar);
        }
        break;
    }

    ++position;
    ++linePosition;
  }

  return lexemes;
}

const static std::string source = "a = 2\n"
                                  ";\n"
                                  "\n"
                                  "b = times a 2\n"
                                  ";\n";

std::string*
readFile(const std::string& filePath)
{
  std::ifstream file(filePath);
  if (!file.is_open()) {
    throw std::runtime_error("Could not open file");
  }

  // Create a std::stringstream object to read file contents
  std::stringstream buffer;
  buffer << file.rdbuf();

  // Allocate memory on the heap for the string
  std::string* fileContents = new std::string(buffer.str());

  // Return a pointer to the heap-allocated string
  return fileContents;
}

// PARSER

struct VariableImpl
{
  Lexema name;
};

struct NumberImpl
{
  int value;
};

// Forward declarations
struct AssignmentImpl;

struct ApplicationImpl
{
  Lexema name;
  std::variant<VariableImpl, ApplicationImpl*> parameter;
};

using Expression =
  std::variant<AssignmentImpl, VariableImpl, NumberImpl, ApplicationImpl*>;

struct AssignmentImpl
{
  VariableImpl name;
  Expression* definition;
};

// // Define LambdaImpl
// struct LambdaImpl
// {
//   Lexema name;
//   VariableImpl variable;
//   Expression definition;
// };

Expression // TODO
parseExpr(std::unique_ptr<std::vector<Lexema>>& lexemes)
{
  return Expression();
}

void // TODO
expect(LexemaType lexType, std::vector<Lexema>* lexemes)
{
  return;
}

std::unique_ptr<std::vector<Expression>> // TODO
parser(std::unique_ptr<std::vector<Lexema>> lexemes)
{
  auto expressions = std::make_unique<std::vector<Expression>>();

  while (lexemes->size() > 0) {
    auto exrp = parseExpr(lexemes);
    expect(LexemaType::DIVIDOR, lexemes.get());
    expressions->push_back(exrp);
  }

  return expressions;
};

// TODO Separate code into different modules
// TODO Generate tests Separately

int
main()
{
  // static example
  {
    auto lexemes = lexematize(&source);
    printLexemes(lexemes.get());
  }

  // file example
  {
    const std::string filePath = "./example.smallang";
    std::cout << "\n\nINFO: Tokenizing: " << filePath << std::endl;

    try {
      std::string* fileContent = readFile(filePath);
      auto lexemes = lexematize(fileContent);
      printLexemes(lexemes.get());
    } catch (const std::exception& e) {
      std::cerr << e.what() << std::endl;
      return 1;
    }
  }
  return 0;
}
