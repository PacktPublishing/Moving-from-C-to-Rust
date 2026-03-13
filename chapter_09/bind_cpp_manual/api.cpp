#include "toml.hpp"

extern "C" void *toml_parse_file(const char *path) {
    toml::parse_result result = toml::parse_file(path);
    if (result) {
        return new toml::table(std::move(result).table());
    } else {
        return nullptr;
    }
}

extern "C" void toml_free_table(void *tbl) {
    delete static_cast<toml::table*>(tbl);
}
