import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import "../contextmenu"

Flickable {
    id: appFlick
    anchors.fill: parent
    contentWidth: width
    contentHeight: appColumn.implicitHeight + 40
    clip: true
    interactive: true
    boundsBehavior: Flickable.StopAtBounds
    ScrollBar.vertical: ScrollBar {
        policy: ScrollBar.AsNeeded
        width: 4
        anchors.right: parent.right 
        anchors.rightMargin: 2
        z: 1
        
        background: Rectangle { implicitWidth: 4; color: theme.colormap.bgmain; opacity: 0.0 }
        contentItem: Rectangle {
            implicitWidth: 4
            radius: 2
            color: theme.colormap.playeraccent
            Behavior on color { ColorAnimation { duration: 200 } }
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

        ColumnLayout {
            Layout.alignment: Qt.AlignHCenter 
            spacing: 4

            Repeater {
                id: customThemeRepeater
                model: theme.get_custom_theme_count()

                delegate: Rectangle {
                    property int slotIndex: index
                    property string slotName: theme.get_custom_theme_name(index)

                    Layout.preferredWidth: 200
                    Layout.preferredHeight: 32
                    Layout.alignment: Qt.AlignHCenter 
                    radius: 4
                    color: String(index + 1) === theme.current_theme ? theme.colormap["playeraccent"] : theme.colormap["bgoverlay"]
                    border.color: {
                        if (root.appearanceContextMenuVisible && root.appearanceContextMenuIndex === slotIndex) {
                            return theme.colormap["playeraccent"]
                        }
                        if (customItemArea.containsMouse) {
                            return theme.colormap["playeraccent"]
                        }
                        return theme.colormap["graysolid"]
                    }
                    border.width: (root.appearanceContextMenuVisible && root.appearanceContextMenuIndex === slotIndex) ? 2 : 1

                    Behavior on color { ColorAnimation { duration: 150 } }
                    Behavior on border.color { ColorAnimation { duration: 150 } }

                    Text {
                        anchors.centerIn: parent
                        text: slotName
                        font.family: kodeMono.name
                        font.pixelSize: 12
                        color: String(index + 1) === theme.current_theme ? theme.colormap["bgmain"] : theme.colormap["playlisttext"]
                        font.bold: String(index + 1) === theme.current_theme
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
                                root.appearanceContextMenuIndex = slotIndex
                                root.appearanceContextMenuVisible = true
                                root.playlistContextMenuVisible = false
                            } else if (mouse.button === Qt.LeftButton) {
                                root.appearanceContextMenuVisible = false
                                theme.set_theme(String(slotIndex + 1))
                            }
                        }
                    }
                }
            }
        }

        Item { Layout.preferredHeight: 32 }

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
                        root.themeEditorSlotTarget = -1
                        root.themeEditorVisible = true
                    }
                }
            }
        }

        Item { Layout.preferredHeight: 20 }
    }

    Item { Layout.fillHeight: true }
}