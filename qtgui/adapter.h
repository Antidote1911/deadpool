#ifndef ADAPTER_H
#define ADAPTER_H

#include <QFileDialog>
#include <QString>

// rust functions
extern "C" char* get_version();

extern "C" char* get_random(int length,
                            bool uppercase,
                            bool lowercase,
                            bool numbers,
                            bool braces,
                            bool punctuation,
                            bool quotes,
                            bool dashes,
                            bool math,
                            bool logograms,
                            const char* avoid,
                            const char* also);



#endif // ADAPTER_H
