#include <string>
#include <wx/defs.h>
#include <wx/event.h>
#include <wx/frame.h>
#include <wx/gdicmn.h>
#include <wx/button.h>
#include <wx/listbase.h>
#include <wx/listctrl.h>
#include <wx/menu.h>
#include <wx/msgdlg.h>
#include <wx/sizer.h>

#include "globals.hpp"
#include "state.hpp"
#include "add_volume_dialog.hpp"

class MainWindow : public wxFrame {
    public:
        MainWindow();
    
    private:
        State* state;
        wxListCtrl* listWidget;

        void exit(wxCommandEvent& event);
        void showAbout(wxCommandEvent& event);
        void showAddVolumeDialog(wxCommandEvent& event);
};

inline MainWindow::MainWindow() : wxFrame(NULL, wxID_ANY, APP_NAME) {
    // ====== WINDOW SETUP ====== //
    SetSize(100, 100, 600, 500, wxSIZE_AUTO);

    // ====== STATE SETUP ====== //
    state = new State();

    // ====== WINDOW MENUS ====== //
    wxMenu* fileMenu = new wxMenu;
    fileMenu->Append(wxID_EXIT);

    wxMenu* helpMenu = new wxMenu;
    helpMenu->Append(wxID_ABOUT);

    wxMenuBar* menuBar = new wxMenuBar;
    menuBar->Append(fileMenu, "&File");
    menuBar->Append(helpMenu, "&Help");

    SetMenuBar(menuBar);

    // ====== CONTENT ====== //
    wxBoxSizer* container = new wxBoxSizer(wxHORIZONTAL);
    wxBoxSizer* buttonContainer = new wxBoxSizer(wxVERTICAL);

    listWidget = new wxListCtrl(this, wxID_ANY, wxDefaultPosition, wxDefaultSize, wxLC_REPORT);

    wxListItem columnPathname;
    columnPathname.SetId(0);
    columnPathname.SetText(_("File"));
    columnPathname.SetWidth(200);
    listWidget->InsertColumn(0, columnPathname);

    wxListItem columnVolumeType;
    columnVolumeType.SetId(1);
    columnVolumeType.SetText(_("Type"));
    columnVolumeType.SetWidth(100);
    listWidget->InsertColumn(1, columnVolumeType);

    container->Add(listWidget, 1, wxEXPAND | wxALL, 10);

    wxButton* addButton = new wxButton(this, wxID_ANY, "Add");
    wxButton* modifyButton = new wxButton(this, wxID_ANY, "Modify");
    modifyButton->Disable();
    wxButton* removeButton = new wxButton(this, wxID_ANY, "Remove");
    removeButton->Disable();

    buttonContainer->Add(addButton);
    buttonContainer->AddSpacer(5);
    buttonContainer->Add(modifyButton);
    buttonContainer->AddSpacer(20);
    buttonContainer->Add(removeButton);

    container->Add(buttonContainer);

    SetSizer(container);

    // ====== EVENT BINDING ====== //
    Bind(wxEVT_MENU, &MainWindow::showAbout, this, wxID_ABOUT);
    Bind(wxEVT_MENU, &MainWindow::exit, this, wxID_EXIT);
    
    addButton->Bind(wxEVT_BUTTON, &MainWindow::showAddVolumeDialog, this, wxID_ANY);
}

inline void MainWindow::showAddVolumeDialog(wxCommandEvent& event) {
    AddVolumeDialog* dialog = new AddVolumeDialog("Volume info");
    dialog->Show(true);
}

inline void MainWindow::exit(wxCommandEvent& event) {
    Close(true);
}

inline void MainWindow::showAbout(wxCommandEvent& event) {
    std::string message = "This is a small utility for managing volumes";
    std::string popupTitle = "About Voxy";
    wxMessageBox(message, popupTitle, wxOK | wxICON_INFORMATION);
}