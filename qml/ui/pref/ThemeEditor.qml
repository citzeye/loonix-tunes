/* --- loonix-tunes/qml/ui/pref/ThemeEditor.qml --- */
import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import QtQuick.Dialogs

Item {
    id: themeEditorRoot
    anchors.fill: parent
    z: 20000
    visible: root.themeEditorVisible

    property int refreshTicker: 0
    property int selectedProfileIndex: 0

    Connections {
        target: theme
        function onCustom_themes_changed() {
            refreshTicker++;
        }
    }

    MouseArea {
        anchors.fill: parent
        onClicked: {
            root.themeEditorVisible = false;
        }
    }

    function scanCurrentEditorColors() {
        return {
            "bgmain": inBgMain.inputText,
            "bgoverlay": inBgOverlay.inputText,
            "graysolid": inGraySolid.inputText,
            "contextmenubg": inContextMenuBg.inputText,
            "overlay": inOverlay.inputText,
            "headerbg": inHeaderBg.inputText,
            "headericon": inHeaderIcon.inputText,
            "headertext": inHeaderText.inputText,
            "headerhover": inHeaderHover.inputText,
            "playertitle": inPlayerTitle.inputText,
            "playersubtext": inPlayerSubtext.inputText,
            "playeraccent": inPlayerAccent.inputText,
            "playerhover": inPlayerHover.inputText,
            "tabtext": inTabText.inputText,
            "tabborder": inTabBorder.inputText,
            "tabhover": inTabHover.inputText,
            "playlisttext": inPlaylistText.inputText,
            "playlistfolder": inPlaylistFolder.inputText,
            "playlistactive": inPlaylistActive.inputText,
            "playlisticon": inPlaylistIcon.inputText,
            "eqbg": inEqBg.inputText,
            "eqborder": inEqBorder.inputText,
            "eqtext": inEqText.inputText,
            "eqsubtext": inEqSubtext.inputText,
            "eqicon": inEqIcon.inputText,
            "eqhover": inEqHover.inputText,
            "eqactive": inEqActive.inputText,
            "eqsliderbg": inEqSliderBg.inputText,
            "eqfader": inEqFader.inputText,
            "eqmix": inEqMix.inputText,
            "eqhandle": inEqHandle.inputText,
            "fxbg": inFxBg.inputText,
            "fxborder": inFxBorder.inputText,
            "fxtext": inFxText.inputText,
            "fxsubtext": inFxSubtext.inputText,
            "fxicon": inFxIcon.inputText,
            "fxhover": inFxHover.inputText,
            "fxactive": inFxActive.inputText,
            "fxslider": inFxSlider.inputText,
            "fxsliderbg": inFxSliderBg.inputText,
            "fxhandle": inFxHandle.inputText
        };
    }

    onVisibleChanged: {
        if (visible) {
            themeEditorRoot.selectedProfileIndex = root.themeEditorProfileTarget >= 0 ? root.themeEditorProfileTarget : 0;

            if (root.themeEditorProfileTarget >= 0) {
                // EDIT mode
                var savedColors = theme.get_custom_theme_colors(root.themeEditorProfileTarget);
                inBgMain.inputText = savedColors.bgmain;
                inBgOverlay.inputText = savedColors.bgoverlay;
                inGraySolid.inputText = savedColors.graysolid;
                inContextMenuBg.inputText = savedColors.contextmenubg;
                inOverlay.inputText = savedColors.overlay;
                inHeaderBg.inputText = savedColors.headerbg;
                inHeaderIcon.inputText = savedColors.headericon;
                inHeaderText.inputText = savedColors.headertext;
                inHeaderHover.inputText = savedColors.headerhover;
                inPlayerTitle.inputText = savedColors.playertitle;
                inPlayerSubtext.inputText = savedColors.playersubtext;
                inPlayerAccent.inputText = savedColors.playeraccent;
                inPlayerHover.inputText = savedColors.playerhover;
                inTabText.inputText = savedColors.tabtext;
                inTabBorder.inputText = savedColors.tabborder;
                inTabHover.inputText = savedColors.tabhover;
                inPlaylistText.inputText = savedColors.playlisttext;
                inPlaylistFolder.inputText = savedColors.playlistfolder;
                inPlaylistActive.inputText = savedColors.playlistactive;
                inPlaylistIcon.inputText = savedColors.playlisticon;
                inEqBg.inputText = savedColors.eqbg;
                inEqBorder.inputText = savedColors.eqborder;
                inEqText.inputText = savedColors.eqtext;
                inEqSubtext.inputText = savedColors.eqsubtext;
                inEqIcon.inputText = savedColors.eqicon;
                inEqHover.inputText = savedColors.eqhover;
                inEqActive.inputText = savedColors.eqactive;
                inEqSliderBg.inputText = savedColors.eqsliderbg;
                inEqFader.inputText = savedColors.eqfader;
                inEqMix.inputText = savedColors.eqmix;
                inEqHandle.inputText = savedColors.eqhandle;
                inFxBg.inputText = savedColors.fxbg;
                inFxBorder.inputText = savedColors.fxborder;
                inFxText.inputText = savedColors.fxtext;
                inFxSubtext.inputText = savedColors.fxsubtext;
                inFxIcon.inputText = savedColors.fxicon;
                inFxHover.inputText = savedColors.fxhover;
                inFxActive.inputText = savedColors.fxactive;
                inFxSlider.inputText = savedColors.fxslider;
                inFxSliderBg.inputText = savedColors.fxsliderbg;
                inFxHandle.inputText = savedColors.fxhandle;
            } else {
                // CREATE mode
                inBgMain.inputText = theme.colormap.bgmain;
                inBgOverlay.inputText = theme.colormap.bgoverlay;
                inGraySolid.inputText = theme.colormap.graysolid;
                inContextMenuBg.inputText = theme.colormap.contextmenubg;
                inOverlay.inputText = theme.colormap.overlay;
                inHeaderBg.inputText = theme.colormap.headerbg;
                inHeaderIcon.inputText = theme.colormap.headericon;
                inHeaderText.inputText = theme.colormap.headertext;
                inHeaderHover.inputText = theme.colormap.headerhover;
                inPlayerTitle.inputText = theme.colormap.playertitle;
                inPlayerSubtext.inputText = theme.colormap.playersubtext;
                inPlayerAccent.inputText = theme.colormap.playeraccent;
                inPlayerHover.inputText = theme.colormap.playerhover;
                inTabText.inputText = theme.colormap.tabtext;
                inTabBorder.inputText = theme.colormap.tabborder;
                inTabHover.inputText = theme.colormap.tabhover;
                inPlaylistText.inputText = theme.colormap.playlisttext;
                inPlaylistFolder.inputText = theme.colormap.playlistfolder;
                inPlaylistActive.inputText = theme.colormap.playlistactive;
                inPlaylistIcon.inputText = theme.colormap.playlisticon;
                inEqBg.inputText = theme.colormap.eqbg;
                inEqBorder.inputText = theme.colormap.eqborder;
                inEqText.inputText = theme.colormap.eqtext;
                inEqSubtext.inputText = theme.colormap.eqsubtext;
                inEqIcon.inputText = theme.colormap.eqicon;
                inEqHover.inputText = theme.colormap.eqhover;
                inEqActive.inputText = theme.colormap.eqactive;
                inEqSliderBg.inputText = theme.colormap.eqsliderbg;
                inEqFader.inputText = theme.colormap.eqfader;
                inEqMix.inputText = theme.colormap.eqmix;
                inEqHandle.inputText = theme.colormap.eqhandle;
                inFxBg.inputText = theme.colormap.fxbg;
                inFxBorder.inputText = theme.colormap.fxborder;
                inFxText.inputText = theme.colormap.fxtext;
                inFxSubtext.inputText = theme.colormap.fxsubtext;
                inFxIcon.inputText = theme.colormap.fxicon;
                inFxHover.inputText = theme.colormap.fxhover;
                inFxActive.inputText = theme.colormap.fxactive;
                inFxSlider.inputText = theme.colormap.fxslider;
                inFxSliderBg.inputText = theme.colormap.fxsliderbg;
                inFxHandle.inputText = theme.colormap.fxhandle;
            }
        } else {
            root.themeEditorProfileTarget = -1;
        }
    }

    Rectangle {
        width: 420
        height: 520
        anchors.centerIn: parent
        color: theme.colormap.bgmain
        border.color: theme.colormap.tabborder
        border.width: 1
        radius: 4

        MouseArea {
            anchors.fill: parent
            acceptedButtons: Qt.AllButtons
            propagateComposedEvents: true
            onClicked: mouse.accepted = false
        }

        ColumnLayout {
            anchors.fill: parent
            anchors.margins: 12
            spacing: 8

            // --- HEADER ---
            RowLayout {
                Layout.fillWidth: true
                Text {
                    text: "THEME EDITOR"
                    color: theme.colormap.playeraccent
                    font.family: kodeMono.name
                    font.pixelSize: 14
                    font.bold: true
                    Layout.fillWidth: true
                }

                Text {
                    id: closeBtn
                    text: "󰅖"
                    font.family: symbols.name
                    font.pixelSize: 16
                    color: closeMA.containsMouse ? theme.colormap.playerhover : theme.colormap.tabtext

                    MouseArea {
                        id: closeMA
                        anchors.fill: parent
                        anchors.margins: -10
                        hoverEnabled: true
                        onClicked: root.themeEditorVisible = false
                    }
                }
            }

            // --- THEME NAME INPUT ---
            RowLayout {
                Layout.fillWidth: true
                spacing: 8

                Text {
                    text: "NAME"
                    color: theme.colormap.tabtext
                    font.family: kodeMono.name
                    font.pixelSize: 11
                }

                TextField {
                    id: themeNameInput
                    Layout.fillWidth: true
                    Layout.preferredHeight: 28
                    text: root.themeEditorProfileTarget >= 0 ? theme.get_custom_theme_name(root.themeEditorProfileTarget) : (themeEditorRoot.selectedProfileIndex >= 0 ? theme.get_custom_theme_name(themeEditorRoot.selectedProfileIndex) : "New Theme")
                    color: theme.colormap.playeraccent
                    font.family: kodeMono.name
                    font.pixelSize: 12
                    background: Rectangle {
                        color: theme.colormap.bgoverlay
                        radius: 4
                        border.color: theme.colormap.tabborder
                        border.width: 1
                    }
                }
            }

            Rectangle {
                Layout.fillWidth: true
                Layout.preferredHeight: 1
                color: theme.colormap.tabborder
                Layout.topMargin: 4
                Layout.bottomMargin: 4
            }

            // --- SCROLLABLE AREA (Radio Buttons + Colors) ---
            ScrollView {
                Layout.fillWidth: true
                Layout.fillHeight: true
                clip: true
                ScrollBar.horizontal.policy: ScrollBar.AlwaysOff

                // OVERRIDE SCROLLBAR VERTIKAL BIAR MODERN
                ScrollBar.vertical: ScrollBar {
                    width: 6
                    policy: ScrollBar.AsNeeded

                    contentItem: Rectangle {
                        implicitWidth: 6
                        radius: 3
                        color: theme.colormap.tabborder // Pake warna border biar nyambung
                        opacity: parent.pressed ? 1.0 : (parent.hovered ? 0.8 : 0.3)

                        Behavior on opacity {
                            NumberAnimation {
                                duration: 150
                            }
                        }
                    }

                    background: Rectangle {
                        color: "transparent"
                    }
                }

                ColumnLayout {
                    width: parent.width - 58
                    spacing: 6

                    // --- RADIO BUTTONS (Sekarang ikut scroll) ---
                    ColumnLayout {
                        Layout.fillWidth: true
                        Layout.bottomMargin: 8
                        spacing: 2

                        Text {
                            text: "Choose which theme to replace:"
                            color: theme.colormap.tabtext
                            font.family: kodeMono.name
                            font.pixelSize: 10
                        }

                        Repeater {
                            model: theme.get_custom_theme_count()
                            delegate: RadioButton {
                                text: theme.get_custom_theme_name(index)
                                checked: themeEditorRoot.selectedProfileIndex === index
                                onClicked: {
                                    themeEditorRoot.selectedProfileIndex = index;
                                    root.themeEditorProfileTarget = index;
                                }
                                indicator.width: 14
                                indicator.height: 14

                                // Override contentItem buat masukin warna
                                contentItem: Text {
                                    text: parent.text
                                    color: theme.colormap.tabtext
                                    font.family: kodeMono.name
                                    font.pixelSize: 11
                                    leftPadding: parent.indicator.width + parent.spacing
                                    verticalAlignment: Text.AlignVCenter
                                }
                            }
                        }
                    }

                    Rectangle {
                        Layout.fillWidth: true
                        Layout.preferredHeight: 1
                        color: theme.colormap.tabborder
                        Layout.bottomMargin: 8
                    }

                    // === BACKGROUNDS ===
                    SectionHeader {
                        id: sh1
                        sectionTitle: "BACKGROUNDS"
                    }
                    ColorInputRow {
                        id: inBgMain
                        labelText: "bgmain"
                        hexValue: theme.colormap.bgmain
                    }
                    ColorInputRow {
                        id: inBgOverlay
                        labelText: "bgoverlay"
                        hexValue: theme.colormap.bgoverlay
                    }
                    ColorInputRow {
                        id: inGraySolid
                        labelText: "graysolid"
                        hexValue: theme.colormap.graysolid
                    }
                    ColorInputRow {
                        id: inContextMenuBg
                        labelText: "contextmenubg"
                        hexValue: theme.colormap.contextmenubg
                    }
                    ColorInputRow {
                        id: inOverlay
                        labelText: "overlay"
                        hexValue: theme.colormap.overlay
                    }

                    // === HEADER ===
                    SectionHeader {
                        id: sh2
                        sectionTitle: "HEADER"
                    }
                    ColorInputRow {
                        id: inHeaderBg
                        labelText: "headerbg"
                        hexValue: theme.colormap.headerbg
                    }
                    ColorInputRow {
                        id: inHeaderIcon
                        labelText: "headericon"
                        hexValue: theme.colormap.headericon
                    }
                    ColorInputRow {
                        id: inHeaderText
                        labelText: "headertext"
                        hexValue: theme.colormap.headertext
                    }
                    ColorInputRow {
                        id: inHeaderHover
                        labelText: "headerhover"
                        hexValue: theme.colormap.headerhover
                    }

                    // === PLAYER ===
                    SectionHeader {
                        id: sh3
                        sectionTitle: "PLAYER"
                    }
                    ColorInputRow {
                        id: inPlayerTitle
                        labelText: "playertitle"
                        hexValue: theme.colormap.playertitle
                    }
                    ColorInputRow {
                        id: inPlayerSubtext
                        labelText: "playersubtext"
                        hexValue: theme.colormap.playersubtext
                    }
                    ColorInputRow {
                        id: inPlayerAccent
                        labelText: "playeraccent"
                        hexValue: theme.colormap.playeraccent
                    }
                    ColorInputRow {
                        id: inPlayerHover
                        labelText: "playerhover"
                        hexValue: theme.colormap.playerhover
                    }

                    // === TABS ===
                    SectionHeader {
                        id: sh4
                        sectionTitle: "TABS"
                    }
                    ColorInputRow {
                        id: inTabText
                        labelText: "tabtext"
                        hexValue: theme.colormap.tabtext
                    }
                    ColorInputRow {
                        id: inTabBorder
                        labelText: "tabborder"
                        hexValue: theme.colormap.tabborder
                    }
                    ColorInputRow {
                        id: inTabHover
                        labelText: "tabhover"
                        hexValue: theme.colormap.tabhover
                    }

                    // === PLAYLIST ===
                    SectionHeader {
                        id: sh5
                        sectionTitle: "PLAYLIST"
                    }
                    ColorInputRow {
                        id: inPlaylistText
                        labelText: "playlisttext"
                        hexValue: theme.colormap.playlisttext
                    }
                    ColorInputRow {
                        id: inPlaylistFolder
                        labelText: "playlistfolder"
                        hexValue: theme.colormap.playlistfolder
                    }
                    ColorInputRow {
                        id: inPlaylistActive
                        labelText: "playlistactive"
                        hexValue: theme.colormap.playlistactive
                    }
                    ColorInputRow {
                        id: inPlaylistIcon
                        labelText: "playlisticon"
                        hexValue: theme.colormap.playlisticon
                    }

                    // === EQ ===
                    SectionHeader {
                        id: sh6
                        sectionTitle: "EQ"
                    }
                    ColorInputRow {
                        id: inEqBg
                        labelText: "eqbg"
                        hexValue: theme.colormap.eqbg
                    }
                    ColorInputRow {
                        id: inEqBorder
                        labelText: "eqborder"
                        hexValue: theme.colormap.eqborder
                    }
                    ColorInputRow {
                        id: inEqText
                        labelText: "eqtext"
                        hexValue: theme.colormap.eqtext
                    }
                    ColorInputRow {
                        id: inEqSubtext
                        labelText: "eqsubtext"
                        hexValue: theme.colormap.eqsubtext
                    }
                    ColorInputRow {
                        id: inEqIcon
                        labelText: "eqicon"
                        hexValue: theme.colormap.eqicon
                    }
                    ColorInputRow {
                        id: inEqHover
                        labelText: "eqhover"
                        hexValue: theme.colormap.eqhover
                    }
                    ColorInputRow {
                        id: inEqActive
                        labelText: "eqactive"
                        hexValue: theme.colormap.eqactive
                    }
                    ColorInputRow {
                        id: inEqSliderBg
                        labelText: "eqsliderbg"
                        hexValue: theme.colormap.eqsliderbg
                    }
                    ColorInputRow {
                        id: inEqFader
                        labelText: "eqfader"
                        hexValue: theme.colormap.eqfader
                    }
                    ColorInputRow {
                        id: inEqMix
                        labelText: "eqmix"
                        hexValue: theme.colormap.eqmix
                    }
                    ColorInputRow {
                        id: inEqHandle
                        labelText: "eqhandle"
                        hexValue: theme.colormap.eqhandle
                    }

                    // === FX ===
                    SectionHeader {
                        id: sh7
                        sectionTitle: "FX"
                    }
                    ColorInputRow {
                        id: inFxBg
                        labelText: "fxbg"
                        hexValue: theme.colormap.fxbg
                    }
                    ColorInputRow {
                        id: inFxBorder
                        labelText: "fxborder"
                        hexValue: theme.colormap.fxborder
                    }
                    ColorInputRow {
                        id: inFxText
                        labelText: "fxtext"
                        hexValue: theme.colormap.fxtext
                    }
                    ColorInputRow {
                        id: inFxSubtext
                        labelText: "fxsubtext"
                        hexValue: theme.colormap.fxsubtext
                    }
                    ColorInputRow {
                        id: inFxIcon
                        labelText: "fxicon"
                        hexValue: theme.colormap.fxicon
                    }
                    ColorInputRow {
                        id: inFxHover
                        labelText: "fxhover"
                        hexValue: theme.colormap.fxhover
                    }
                    ColorInputRow {
                        id: inFxActive
                        labelText: "fxactive"
                        hexValue: theme.colormap.fxactive
                    }
                    ColorInputRow {
                        id: inFxSlider
                        labelText: "fxslider"
                        hexValue: theme.colormap.fxslider
                    }
                    ColorInputRow {
                        id: inFxSliderBg
                        labelText: "fxsliderbg"
                        hexValue: theme.colormap.fxsliderbg
                    }
                    ColorInputRow {
                        id: inFxHandle
                        labelText: "fxhandle"
                        hexValue: theme.colormap.fxhandle
                    }
                }
            }

            // --- BOTTOM BUTTONS (Satu Baris, Lebih Pendek, Tetap Nempel Bawah) ---
            RowLayout {
                Layout.fillWidth: true
                Layout.topMargin: 4
                spacing: 8

                Rectangle {
                    Layout.fillWidth: true
                    Layout.preferredHeight: 28  // Lebih compact
                    radius: 4
                    color: cancelBtnArea.containsMouse ? theme.colormap.graysolid : theme.colormap.bgoverlay
                    border.color: theme.colormap.tabborder
                    border.width: 1

                    Text {
                        anchors.centerIn: parent
                        text: "CANCEL"
                        color: cancelBtnArea.containsMouse ? theme.colormap.bgmain : theme.colormap.tabtext
                        font.family: kodeMono.name
                        font.pixelSize: 11
                        font.bold: true
                    }

                    MouseArea {
                        id: cancelBtnArea
                        anchors.fill: parent
                        hoverEnabled: true
                        onClicked: root.themeEditorVisible = false
                    }
                }

                Rectangle {
                    Layout.fillWidth: true
                    Layout.preferredHeight: 28 // Lebih compact
                    radius: 4
                    color: resetBtnArea.containsMouse ? theme.colormap.graysolid : theme.colormap.bgoverlay
                    border.color: theme.colormap.tabborder
                    border.width: 1

                    Text {
                        anchors.centerIn: parent
                        text: "RESET"
                        color: resetBtnArea.containsMouse ? theme.colormap.bgmain : theme.colormap.tabtext
                        font.family: kodeMono.name
                        font.pixelSize: 11
                        font.bold: true
                    }

                    MouseArea {
                        id: resetBtnArea
                        anchors.fill: parent
                        hoverEnabled: true
                        onClicked: {
                            var defaults = theme.get_default_colors();
                            inBgMain.inputText = defaults.bgmain;
                            inBgOverlay.inputText = defaults.bgoverlay;
                            inGraySolid.inputText = defaults.graysolid;
                            inContextMenuBg.inputText = defaults.contextmenubg;
                            inOverlay.inputText = defaults.overlay;
                            inHeaderBg.inputText = defaults.headerbg;
                            inHeaderIcon.inputText = defaults.headericon;
                            inHeaderText.inputText = defaults.headertext;
                            inHeaderHover.inputText = defaults.headerhover;
                            inPlayerTitle.inputText = defaults.playertitle;
                            inPlayerSubtext.inputText = defaults.playersubtext;
                            inPlayerAccent.inputText = defaults.playeraccent;
                            inPlayerHover.inputText = defaults.playerhover;
                            inTabText.inputText = defaults.tabtext;
                            inTabBorder.inputText = defaults.tabborder;
                            inTabHover.inputText = defaults.tabhover;
                            inPlaylistText.inputText = defaults.playlisttext;
                            inPlaylistFolder.inputText = defaults.playlistfolder;
                            inPlaylistActive.inputText = defaults.playlistactive;
                            inPlaylistIcon.inputText = defaults.playlisticon;
                            inEqBg.inputText = defaults.eqbg;
                            inEqBorder.inputText = defaults.eqborder;
                            inEqText.inputText = defaults.eqtext;
                            inEqSubtext.inputText = defaults.eqsubtext;
                            inEqIcon.inputText = defaults.eqicon;
                            inEqHover.inputText = defaults.eqhover;
                            inEqActive.inputText = defaults.eqactive;
                            inEqSliderBg.inputText = defaults.eqsliderbg;
                            inEqFader.inputText = defaults.eqfader;
                            inEqMix.inputText = defaults.eqmix;
                            inEqHandle.inputText = defaults.eqhandle;
                            inFxBg.inputText = defaults.fxbg;
                            inFxBorder.inputText = defaults.fxborder;
                            inFxText.inputText = defaults.fxtext;
                            inFxSubtext.inputText = defaults.fxsubtext;
                            inFxIcon.inputText = defaults.fxicon;
                            inFxHover.inputText = defaults.fxhover;
                            inFxActive.inputText = defaults.fxactive;
                            inFxSlider.inputText = defaults.fxslider;
                            inFxSliderBg.inputText = defaults.fxsliderbg;
                            inFxHandle.inputText = defaults.fxhandle;
                        }
                    }
                }

                Rectangle {
                    Layout.fillWidth: true
                    Layout.preferredHeight: 28 // Lebih compact
                    radius: 4
                    color: saveThemeMA.containsMouse ? theme.colormap.playeraccent : theme.colormap.bgoverlay
                    border.color: theme.colormap.playeraccent
                    border.width: 1

                    Behavior on color {
                        ColorAnimation {
                            duration: 150
                        }
                    }

                    Text {
                        anchors.centerIn: parent
                        text: "SAVE"
                        color: saveThemeMA.containsMouse ? theme.colormap.bgmain : theme.colormap.playeraccent
                        font.family: kodeMono.name
                        font.pixelSize: 11
                        font.bold: true
                    }

                    MouseArea {
                        id: saveThemeMA
                        anchors.fill: parent
                        hoverEnabled: true
                        onClicked: {
                            // 1. CEK KONDISI SEBELUM NAMA DIUBAH
                            // Kita cari tahu: "Apakah yang mau ditimpa ini tema yang lagi nempel di layar?"
                            var oldProfileName = theme.get_custom_theme_name(themeEditorRoot.selectedProfileIndex)
                            var isCurrentlyActive = (oldProfileName === theme.current_theme)
                            
                            // Cek apakah ini mode CREATE THEME (bukan klik kanan edit)
                            var isCreateMode = (root.themeEditorProfileTarget === -1)
                            
                            var newName = themeNameInput.text

                            // 2. TEMBAK DATA KE RUST
                            theme.set_custom_theme_name(themeEditorRoot.selectedProfileIndex, newName)
                            theme.set_custom_theme_colors(themeEditorRoot.selectedProfileIndex, scanCurrentEditorColors())

                            // 3. LOGIKA AUTO-APPLY:
                            // Kalau kita ngedit tema yang SEDANG AKTIF, atau kita BIKIN TEMA BARU, paksa UI ganti!
                            // Tapi kalau kita diem-diem ngedit tema cadangan di background, biarin aja (gak di-apply).
                            if (isCurrentlyActive || isCreateMode) {
                                theme.set_theme(newName)
                            }
                            
                            root.themeEditorVisible = false
                        }
                    }
                }
            }
        }
    }

    // KOMPONEN DI-DEFINE DI DALAM SINI (Aman)
    component ColorInputRow: RowLayout {
        property string labelText: "Color"
        property string hexValue: "#000000"
        property alias inputText: hexField.text

        ColorDialog {
            id: colorPicker
            title: "Select " + labelText
            selectedColor: hexField.text
            options: ColorDialog.ShowAlphaChannel
            onAccepted: {
                hexField.text = colorPicker.selectedColor.toString();
            }
        }

        Label {
            text: labelText
            color: theme.colormap.tabtext
            font.family: kodeMono.name
            font.pixelSize: 10
            Layout.preferredWidth: 100
        }

        Rectangle {
            width: 20
            height: 20
            radius: 3
            color: hexField.text
            border.color: theme.colormap.tabborder
            border.width: 1

            MouseArea {
                anchors.fill: parent
                hoverEnabled: true
                cursorShape: Qt.PointingHandCursor
                onClicked: colorPicker.open()
            }
        }

        TextField {
            id: hexField
            text: hexValue
            Layout.preferredWidth: 80
            color: theme.colormap.playeraccent
            font.family: kodeMono.name
            font.pixelSize: 11
            horizontalAlignment: TextInput.AlignHCenter
            background: Rectangle {
                color: theme.colormap.bgoverlay
                radius: 3
                border.color: theme.colormap.tabborder
                border.width: 1
            }
        }
    }

    component SectionHeader: Text {
        property string sectionTitle: ""
        text: sectionTitle
        color: theme.colormap.playeraccent
        font.family: kodeMono.name
        font.pixelSize: 12
        font.bold: true
        Layout.fillWidth: true
        Layout.topMargin: 8
        Layout.bottomMargin: 4
    }
}
