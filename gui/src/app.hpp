#include <wx/app.h>
#include "main_window.hpp"

class VoxyApp : public wxApp {
    public:
        virtual bool OnInit();
};

inline bool VoxyApp::OnInit() {
    MainWindow* mainWindow = new MainWindow();
    mainWindow->Show(true);
    return true;
}