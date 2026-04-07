/* --- LOONIX-TUNES qml/ui/contextmenu/AppearanceContextMenu.qml --- */

import QtQuick
import QtQuick.Layouts
import QtQuick.Controls

Item {
  id: appearanceCtxRoot
  z: 10000
  visible: root.appearanceContextMenuVisible
  x: root.appearanceContextMenuX
  y: root.appearanceContextMenuY
  implicitWidth: menuGrid.width + 16
  implicitHeight: menuGrid.height + 16

  Rectangle {
    id: borderRect
    anchors.fill: parent
    anchors.margins: 7
    color: 'transparent'
    radius: 4
    border.color: theme.colormap.tabborder
    border.width: 1
    antialiasing: false
  }

  Rectangle {
    anchors.fill: parent
    anchors.margins: 8
    color: theme.colormap.bgmain
    radius: 4
  }

  MouseArea {
    anchors.fill: parent
    anchors.margins: -10000
    hoverEnabled: true
    acceptedButtons: Qt.LeftButton | Qt.RightButton
    onClicked: {
      root.appearanceContextMenuVisible = false
    }
  }

  GridLayout {
    id: menuGrid
    anchors.centerIn: parent
    columns: 3
    rowSpacing: 2
    columnSpacing: 2

    // TILE 1 - Edit
    Rectangle {
      Layout.preferredWidth: 50
      Layout.preferredHeight: 50
      radius: 4
      color: theme.colormap.bgmain
      Column {
        anchors.centerIn: parent
        spacing: 4
        Text {
          anchors.horizontalCenter: parent.horizontalCenter
          text: ''
          font.family: symbols.name
          font.pixelSize: 18
          color: tile1MA.containsMouse ? theme.colormap.playlisticon : theme.colormap.tabtext
        }
        Text {
          anchors.horizontalCenter: parent.horizontalCenter
          text: 'Edit'
          font.family: kodeMono.name
          font.pixelSize: 10
          color: tile1MA.containsMouse ? theme.colormap.playlisticon : theme.colormap.tabtext
        }
      }
      MouseArea {
        id: tile1MA
        anchors.fill: parent
        hoverEnabled: true
        onClicked: {
          root.appearanceContextMenuVisible = false
          root.themeEditorSlotTarget = root.appearanceContextMenuIndex
          root.themeEditorVisible = true
        }
      }
    }

    // TILE 2 - Rename
    Rectangle {
      Layout.preferredWidth: 50
      Layout.preferredHeight: 50
      radius: 4
      color: theme.colormap.bgmain
      Column {
        anchors.centerIn: parent
        spacing: 4
        Text {
          anchors.horizontalCenter: parent.horizontalCenter
          text: '󰑕'
          font.family: symbols.name
          font.pixelSize: 18
          color: tile2MA.containsMouse ? theme.colormap.playlisticon : theme.colormap.tabtext
        }
        Text {
          anchors.horizontalCenter: parent.horizontalCenter
          text: 'Rename'
          font.family: kodeMono.name
          font.pixelSize: 10
          color: tile2MA.containsMouse ? theme.colormap.playlisticon : theme.colormap.tabtext
        }
      }
      MouseArea {
        id: tile2MA
        anchors.fill: parent
        hoverEnabled: true
        onClicked: {
          root.appearanceContextMenuAction = "rename"
          root.appearanceContextMenuVisible = false
        }
      }
    }

    // TILE 3 - Cancel
    Rectangle {
      Layout.preferredWidth: 50
      Layout.preferredHeight: 50
      radius: 4
      color: theme.colormap.bgmain
      Column {
        anchors.centerIn: parent
        spacing: 4
        Text {
          anchors.horizontalCenter: parent.horizontalCenter
          text: ''
          font.family: symbols.name
          font.pixelSize: 18
          color: theme.colormap.playlistactive
        }
        Text {
          anchors.horizontalCenter: parent.horizontalCenter
          text: 'Cancel'
          font.family: kodeMono.name
          font.pixelSize: 10
          color: theme.colormap.playlistactive
        }
      }
      MouseArea {
        id: tile3MA
        anchors.fill: parent
        hoverEnabled: true
        onClicked: root.appearanceContextMenuVisible = false
      }
    }
  }
}