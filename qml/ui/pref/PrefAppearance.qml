import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import "../contextmenu"

Item {
    id: root
    property int refreshTicker: 0

    Connections {
        target: theme
        function onCustom_themes_changed() {
            refreshTicker++
        }
    }

    Flickable {
        id: appFlick
        anchors.fill: parent
        contentWidth: width
        contentHeight: appColumn.implicitHeight + 40
        clip: true
        interactive: true
        boundsBehavior: Flickable.StopAtBounds

        ScrollBar.vertical: ScrollBar {
            id: vBar
            width: 6
            policy: ScrollBar.AsNeeded
            background: Rectangle { color: "transparent" }
            contentItem: Rectangle {
                implicitWidth: 6
                implicitHeight: Math.max(30, appFlick.height * appFlick.visibleArea.heightRatio)
                radius: 3
                color: vBar.pressed ? theme.colormap.playeraccent : 
                       vBar.hovered ? theme.colormap.headerhover : 
                       theme.colormap.playeraccent
                opacity: vBar.active ? 1.0 : 0.5
                Behavior on color { ColorAnimation { duration: 150 } }
                Behavior on opacity { NumberAnimation { duration: 150 } }
            }
        }

        ColumnLayout {
            id: appColumn
            y: 10
            anchors.leftMargin: 10
            anchors.rightMargin: 10
            anchors.topMargin: 10
            anchors.bottomMargin: 10
            width: appFlick.width - 20
            spacing: 12

            // --- 1. DEFAULT THEMES ---
            ColumnLayout {
                Layout.alignment: Qt.AlignHCenter 
                spacing: 4

                Repeater {
                    model: ["Blue", "Green", "Monochrome", "Orange", "Pink", "Red", "Yellow", "Default"]

                    delegate: Rectangle {
                        Layout.preferredWidth: 200
                        Layout.preferredHeight: 32
                        Layout.alignment: Qt.AlignHCenter 
                        radius: 4
                        color: modelData === theme.current_theme ? theme.colormap["playeraccent"] : theme.colormap["bgoverlay"]
                        border.color: themeItemArea.containsMouse ? theme.colormap["playeraccent"] : theme.colormap["graysolid"]
                        border.width: 1

                        Behavior on color { ColorAnimation { duration: 150 } }
                        Behavior on border.color { ColorAnimation { duration: 150 } }

                        Text {
                            anchors.centerIn: parent
                            text: modelData
                            font.family: kodeMono.name
                            font.pixelSize: 12
                            color: modelData === theme.current_theme ? theme.colormap["bgmain"] : theme.colormap["playlisttext"]
                            font.bold: modelData === theme.current_theme
                        }

                        MouseArea {
                            id: themeItemArea
                            anchors.fill: parent
                            cursorShape: Qt.PointingHandCursor
                            hoverEnabled: true
                            onClicked: theme.set_theme(modelData)
                        }
                    }
                }
            }

            Item { Layout.preferredHeight: 32 }

            // --- 2. CUSTOM THEMES (PRESETS) ---
            ColumnLayout {
                Layout.alignment: Qt.AlignHCenter 
                spacing: 4

                Repeater {
                    id: customThemeRepeater
                    model: theme.get_custom_theme_count()

                    delegate: Rectangle {
                        property int presetIndex: index
                        property string presetName: (refreshTicker, theme.get_custom_theme_name(index))

                        Layout.preferredWidth: 200
                        Layout.preferredHeight: 32
                        Layout.alignment: Qt.AlignHCenter 
                        radius: 4
                        color: presetName === theme.current_theme ? theme.colormap["playeraccent"] : theme.colormap["bgoverlay"]
                        border.color: {
                             if (root.appearanceContextMenuVisible && root.appearanceContextMenuIndex === presetIndex) {
                                return theme.colormap["playeraccent"]
                            }
                            if (customItemArea.containsMouse) {
                                return theme.colormap["playeraccent"]
                            }
                            return theme.colormap["graysolid"]
                        }
                        border.width: (root.appearanceContextMenuVisible && root.appearanceContextMenuIndex === presetIndex) ? 2 : 1

                        Behavior on color { ColorAnimation { duration: 150 } }
                        Behavior on border.color { ColorAnimation { duration: 150 } }

                        Text {
                            anchors.centerIn: parent
                            text: presetName
                            font.family: kodeMono.name
                            font.pixelSize: 12
                            color: presetName === theme.current_theme ? theme.colormap["bgmain"] : theme.colormap["playlisttext"]
                            font.bold: presetName === theme.current_theme
                        }

                        MouseArea {
                            id: customItemArea
                            anchors.fill: parent
                            cursorShape: Qt.PointingHandCursor
                            hoverEnabled: true
                            acceptedButtons: Qt.LeftButton | Qt.RightButton
                            onClicked: (mouse) => {
                                if (mouse.button === Qt.RightButton) {
                                    var mappedPos = customItemArea.mapToItem(root.contentItem, 0, customItemArea.height)
                                    root.appearanceContextMenuX = mappedPos.x
                                    root.appearanceContextMenuY = mappedPos.y
                                    root.appearanceContextMenuIndex = presetIndex
                                    root.appearanceContextMenuVisible = true
                                    root.playlistContextMenuVisible = false
                                } else if (mouse.button === Qt.LeftButton) {
                                    root.appearanceContextMenuVisible = false
                                    theme.set_theme(presetName)
                                }
                            }
                        }
                    }
                }
            }

            Item { Layout.preferredHeight: 32 }

            // --- 3. CREATE THEME BUTTON ---
            ColumnLayout {
                Layout.alignment: Qt.AlignHCenter 

                Rectangle {
                    Layout.preferredWidth: 200
                    Layout.preferredHeight: 32
                    radius: 4
                    color: createThemeArea.containsMouse ? theme.colormap.playeraccent : theme.colormap.bgoverlay
                    border.color: theme.colormap.playeraccent
                    Behavior on color { ColorAnimation { duration: 150 } }

                    Text {
                        anchors.centerIn: parent
                        text: 'CREATE THEME'
                        font.family: kodeMono.name
                        font.pixelSize: 12
                        font.bold: true
                        color: createThemeArea.containsMouse ? theme.colormap.bgmain : theme.colormap.playeraccent
                    }

                    MouseArea {
                        id: createThemeArea
                        anchors.fill: parent
                        cursorShape: Qt.PointingHandCursor
                        hoverEnabled: true
                        onClicked: {
                            root.themeEditorProfileTarget = -1
                            root.themeEditorVisible = true
                        }
                    }
                }
            }

            Item { Layout.preferredHeight: 32 }

            // --- GARIS PEMISAH TEBAL & JELAS ---
            Rectangle {
                Layout.fillWidth: true
                height: 4
                color: theme.colormap.bgmain
                radius: 2 
                Layout.topMargin: 20 
                Layout.bottomMargin: 20 
            }

            /// --- 4. THEME ENGINE (LOONIX VS WALLPAPER) ---
            ColumnLayout {
                Layout.fillWidth: true
                spacing: 16

                Text {
                    Layout.alignment: Qt.AlignHCenter 
                    text: "THEME ENGINE"
                    font.family: kodeMono.name
                    font.pixelSize: 20
                    font.bold: true
                    color: theme.colormap.playerhover
                }

                // Bungkus opsi atas-bawah
                ColumnLayout {
                    Layout.fillWidth: true
                    spacing: 12

                    // --- OPSI 1: LOONIX MANUAL ---
                    Item {
                        Layout.fillWidth: true
                        implicitHeight: loonixRow.implicitHeight
                        opacity: loonixToggle.active ? 1.0 : 0.4
                        Behavior on opacity { NumberAnimation { duration: 200 } }

                        RowLayout {
                            id: loonixRow
                            anchors.fill: parent
                            spacing: 12

                            // Toggle (Lebar Fix)
                            Rectangle {
                                Layout.alignment: Qt.AlignTop 
                                Layout.preferredWidth: 40
                                Layout.preferredHeight: 20
                                radius: 10
                                color: loonixToggle.active ? theme.colormap.playeraccent : theme.colormap.graysolid
                                border.color: theme.colormap.bgoverlay
                                border.width: 2
                                Behavior on color { ColorAnimation { duration: 150 } }

                                Rectangle {
                                    anchors.verticalCenter: parent.verticalCenter
                                    x: loonixToggle.active ? parent.width - 16 - 2 : 2
                                    width: 16
                                    height: 16
                                    radius: 8
                                    color: loonixToggle.active ? theme.colormap.bgmain : theme.colormap.tabtext
                                    Behavior on x { NumberAnimation { duration: 150 } }
                                }
                            }

                            // Teks (Lebar Fleksibel & Word Wrap)
                            ColumnLayout {
                                Layout.fillWidth: true 
                                spacing: 2

                                Text {
                                    Layout.fillWidth: true
                                    wrapMode: Text.WordWrap
                                    text: "Loonix Custom Theme"
                                    font.family: kodeMono.name
                                    font.pixelSize: 11
                                    font.bold: true
                                    color: theme.colormap.tabtext
                                }
                                Text {
                                    Layout.fillWidth: true
                                    wrapMode: Text.WordWrap
                                    text: "Use color scheme from Loonix App."
                                    font.family: kodeMono.name
                                    font.pixelSize: 10
                                    color: theme.colormap.playersubtext
                                }
                            }
                        }

                        // Area klik seluas kotaknya (gak harus ngeklik pas di toggle)
                        MouseArea {
                            id: loonixToggle
                            property bool active: !wallSyncToggle.active
                            anchors.fill: parent
                            cursorShape: Qt.PointingHandCursor
                            onClicked: {
                                wallSyncToggle.active = false
                                active = true
                            }
                        }
                    }

                    // --- OPSI 2: WALLPAPER SYNC ---
                    Item {
                        Layout.fillWidth: true
                        implicitHeight: wallSyncRow.implicitHeight
                        opacity: wallSyncToggle.active ? 1.0 : 0.4
                        Behavior on opacity { NumberAnimation { duration: 200 } }

                        RowLayout {
                            id: wallSyncRow
                            anchors.fill: parent
                            spacing: 12

                            // Toggle (Lebar Fix)
                            Rectangle {
                                Layout.alignment: Qt.AlignTop
                                Layout.preferredWidth: 40
                                Layout.preferredHeight: 20
                                radius: 10
                                color: wallSyncToggle.active ? theme.colormap.playeraccent : theme.colormap.graysolid
                                border.color: theme.colormap.bgoverlay
                                border.width: 2
                                Behavior on color { ColorAnimation { duration: 150 } }

                                Rectangle {
                                    anchors.verticalCenter: parent.verticalCenter
                                    x: wallSyncToggle.active ? parent.width - 16 - 2 : 2
                                    width: 16
                                    height: 16
                                    radius: 8
                                    color: wallSyncToggle.active ? theme.colormap.bgmain : theme.colormap.tabtext
                                    Behavior on x { NumberAnimation { duration: 150 } }
                                }
                            }

                            // Teks (Lebar Fleksibel & Word Wrap)
                            ColumnLayout {
                                Layout.fillWidth: true
                                spacing: 2

                                Text {
                                    Layout.fillWidth: true
                                    wrapMode: Text.WordWrap
                                    text: "Wallpaper Sync (Matugen)"
                                    font.family: kodeMono.name
                                    font.pixelSize: 11
                                    font.bold: true
                                    color: theme.colormap.tabtext
                                }
                                Text {
                                    Layout.fillWidth: true
                                    wrapMode: Text.WordWrap
                                    text: "Use color scheme from device wallpaper."
                                    font.family: kodeMono.name
                                    font.pixelSize: 10
                                    color: theme.colormap.playersubtext
                                }
                            }
                        }

                        // Area klik seluas kotaknya
                        MouseArea {
                            id: wallSyncToggle
                            property bool active: false
                            anchors.fill: parent
                            cursorShape: Qt.PointingHandCursor
                            onClicked: {
                                if (!active) {
                                    active = true
                                    loonixToggle.active = false
                                    theme.sync_with_wallpaper()
                                }
                            }
                        }
                    }
                }
            }

            Item { Layout.preferredHeight: 20 }
            Item { Layout.fillHeight: true } // Spacer bawah biar gak mentok
        }
    }
}