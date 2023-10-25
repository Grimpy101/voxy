
#include <wx/chartype.h>
#include <wx/defs.h>
#include <wx/dialog.h>
#include <wx/gdicmn.h>
#include <wx/generic/textdlgg.h>
#include <wx/stattext.h>
#include <wx/textctrl.h>
#include <wx/listctrl.h>
#include <wx/sizer.h>
#include <wx/string.h>
#include <wx/validate.h>

class AddVolumeDialog : public wxDialog {
    private:
        wxTextCtrl* AddOption(wxBoxSizer* mainContainer, wxString name);
    public:
        AddVolumeDialog(const wxString& title);
};

inline AddVolumeDialog::AddVolumeDialog(const wxString& title)
    : wxDialog(NULL, -1, title, wxDefaultPosition, wxSize(600, 450)) {
    //wxPanel* panel = new wxPanel(this, -1);

    wxBoxSizer* container = new wxBoxSizer(wxHORIZONTAL);
    wxBoxSizer* optionsContainer = new wxBoxSizer(wxVERTICAL);

    wxListView* volumeList = new wxListView(this, wxID_ANY);
    volumeList->m_width = 200;

    wxTextCtrl* nameText = AddOption(optionsContainer, "Name: ");
    wxTextCtrl* generatorText = AddOption(optionsContainer, "Generator: ");
    wxTextCtrl* authorText = AddOption(optionsContainer, "Author: ");
    wxTextCtrl* descriptionText = AddOption(optionsContainer, "Description: ");
    wxTextCtrl* copyrightText = AddOption(optionsContainer, "Copyright: ");
    wxTextCtrl* acquisitionTime = AddOption(optionsContainer, "Acquisition time: ");
    wxTextCtrl* creationTime = AddOption(optionsContainer, "Creation time: ");

    container->Add(volumeList, 1, 0, 5);
    container->AddSpacer(10);
    container->Add(optionsContainer, 0, wxEXPAND, 5);

    SetSizer(container);
    Centre();
    ShowModal();
    Destroy();
}

inline wxTextCtrl* AddVolumeDialog::AddOption(wxBoxSizer* mainContainer, wxString name) {
    wxBoxSizer* container = new wxBoxSizer(wxHORIZONTAL);
    wxStaticText* label = new wxStaticText(this, wxID_ANY, name, wxDefaultPosition, wxDefaultSize, wxALIGN_CENTER_HORIZONTAL | wxALIGN_CENTER_VERTICAL);
    wxTextCtrl* input = new wxTextCtrl(this, wxID_ANY);
    container->Add(label, 1, wxALIGN_CENTER_VERTICAL | wxALIGN_RIGHT);
    container->Add(input, 0, wxEXPAND);

    mainContainer->Add(container, 0, wxEXPAND);

    return input;
}