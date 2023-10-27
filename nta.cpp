#include <iostream>
#include <fstream>
#include <curl/curl.h>
#include <regex>
#include <stdexcept>

struct NtaDownload {
    const char* _repo;
    std::string location;

    NtaDownload() {
        _repo = "https://raw.githubusercontent.com/l1nux-th1ngz/nta/main/";
        location = findNta();
    }

    std::string findNta() {
        // Implement findNta logic here and return the location as a string
        // For example:
        // std::string location = "/path/to/nta_folder";
        // return location;
        // In your actual code, you should implement this function.
        return "/path/to/nta_folder";
    }

    bool downloadLatestVersion() {
        std::cout << "Downloading the latest version of Nta..." << std::endl;

        CURL* curl;
        CURLcode res;

        curl = curl_easy_init();
        if (curl) {
            std::string latestVersionURL = _repo;
            latestVersionURL += "nta.py";

            curl_easy_setopt(curl, CURLOPT_URL, latestVersionURL.c_str());

            std::ofstream latestVersionFile(location);
            curl_easy_setopt(curl, CURLOPT_WRITEDATA, &latestVersionFile);

            res = curl_easy_perform(curl);
            if (res == CURLE_OK) {
                std::cout << "Download complete." << std::endl;
                curl_easy_cleanup(curl);
                latestVersionFile.close();
                return true;
            }

            curl_easy_cleanup(curl);
        }

        return false;
    }
};

int main() {
    std::cout << "Nta Downloader" << std::endl;
    NtaDownload ntaDownload;

    if (ntaDownload.downloadLatestVersion()) {
        // Additional code for the rest of your script goes here.
    } else {
        std::cerr << "Download failed." << std::endl;
        return 1;
    }

    return 0;
}
