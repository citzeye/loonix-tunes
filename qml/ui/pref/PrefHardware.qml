/* --- LOONIX-TUNES qml/ui/pref/PrefHardware.qml --- */

import QtQuick
import QtQuick.Controls
import QtQuick.Layouts

Item {
    id: prefHardware
    width: parent.width
    height: parent.height
    
    Label {
        text: "Hardware Preferences"
        anchors.centerIn: parent
        font.pixelSize: 16
        color: theme.colormap.headertext
    }
}

/* --- END --- */