#include <iostream>
#include <string>

namespace log_line {

const std::string delimiter = ": ";

std::string message(std::string line) {
  // return the message
  return line.substr(line.find(delimiter) + delimiter.size());
}

std::string log_level(std::string line) {
  // return the log level
  return line.substr(1, line.find(delimiter) - delimiter.size());
}

std::string reformat(std::string line) {
  // return the reformatted message
  return message(line) + " (" + log_level(line) + ")";
}
} // namespace log_line
