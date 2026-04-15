/* --- loonix-tunes/qml/ui/Fx.qml --- */
import QtQuick
import QtQuick.Controls
import QtQuick.Layouts

Popup {
    id: fxRoot
    width: 500
    height: implicitHeight
    modal: true
    focus: true
    closePolicy: Popup.CloseOnEscape | Popup.CloseOnPressOutside

    background: Rectangle {
        color: theme.colormap.fxbg
        border.color: theme.colormap.fxborder
        border.width: 1
        radius: 4
        antialiasing: false
    }

    contentItem: Item {
        id: fxContentItem
        anchors.fill: parent

        ColumnLayout {
            anchors.fill: parent
            anchors.margins: 6
            spacing: 4

            // COMPRESSOR
            RowLayout {
                Layout.fillWidth: true
                spacing: 4

                FxToggleBox {
                    id: compToggle
                    title: "COMPRESSOR"
                    isOn: musicModel.compressor_active
                    onToggled: musicModel.toggleStdCompressor()
                }

                FxSliderBox {
                    id: compSlider
                    enabled: compToggle.isOn && musicModel.dsp_enabled
                    controlValue: musicModel.compressor_threshold
                    onSliderChanged: val => musicModel.setStdCompressorThreshold(val)
                }
                FxValueBox {
                    enabled: compToggle.isOn && musicModel.dsp_enabled
                    sliderValue: compSlider.currentValue
                }
                FxResetButton {
                    enabled: compToggle.isOn && musicModel.dsp_enabled
                    useNoArgReset: true
                    onResetNoArg: musicModel.reset_std_compressor()
                }
            }

            // SURROUND
            RowLayout {
                Layout.fillWidth: true
                spacing: 4

                FxToggleBox {
                    id: surrToggle
                    title: "SURROUND"
                    isOn: musicModel.surround_magic_active
                    onToggled: musicModel.toggleStdSurround()
                }

                FxSliderBox {
                    id: surrSlider
                    enabled: surrToggle.isOn && musicModel.dsp_enabled
                    controlValue: musicModel.surround_width
                    onSliderChanged: val => musicModel.setStdSurroundWidth(val * 2.0)
                }
                FxValueBox {
                    enabled: surrToggle.isOn && musicModel.dsp_enabled
                    sliderValue: surrSlider.currentValue
                }
                FxResetButton {
                    enabled: surrToggle.isOn && musicModel.dsp_enabled
                    useNoArgReset: true
                    onResetNoArg: musicModel.reset_std_surround()
                }
            }

            // MONO - STEREO
            RowLayout {
                Layout.fillWidth: true
                spacing: 4

                FxToggleBox {
                    id: monoToggle
                    title: "MONO - STEREO"
                    isOn: musicModel.mono_active
                    onToggled: musicModel.toggleStdStereoWidth()
                }

                FxSliderBox {
                    id: monoSlider
                    enabled: monoToggle.isOn && musicModel.dsp_enabled
                    controlValue: musicModel.mono_width
                    onSliderChanged: val => musicModel.setStdStereoWidthAmount(val)
                }
                FxValueBox {
                    enabled: monoToggle.isOn && musicModel.dsp_enabled
                    sliderValue: monoSlider.controlValue
                }
                FxResetButton {
                    enabled: monoToggle.isOn && musicModel.dsp_enabled
                    useNoArgReset: true
                    onResetNoArg: musicModel.reset_std_stereo_width()
                }
            }

            // MIDDLE CLARITY
            RowLayout {
                Layout.fillWidth: true
                spacing: 4

                FxToggleBox {
                    id: midToggle
                    title: "MIDDLE CLARITY"
                    isOn: musicModel.middle_active
                    onToggled: musicModel.toggleStdMiddleClarity()
                }

                FxSliderBox {
                    id: midSlider
                    enabled: midToggle.isOn
                    controlValue: musicModel.middle_amount
                    onSliderChanged: val => musicModel.setStdMiddleClarityAmount(val)
                }
                FxValueBox {
                    enabled: midToggle.isOn
                    sliderValue: midSlider.controlValue
                }
                FxResetButton {
                    enabled: midToggle.isOn
                    useNoArgReset: true
                    onResetNoArg: musicModel.reset_std_middle_clarity()
                }
            }

            // STEREO ENHANCE
            RowLayout {
                Layout.fillWidth: true
                spacing: 4

                FxToggleBox {
                    id: stereoEnhToggle
                    title: "STEREO ENHANCER"
                    isOn: musicModel.stereo_active
                    onToggled: musicModel.toggleStdStereoEnhance()
                }

                FxSliderBox {
                    id: stereoSlider
                    enabled: stereoEnhToggle.isOn
                    controlValue: musicModel.stereo_amount
                    onSliderChanged: val => musicModel.setStdStereoEnhanceAmount(val)
                }
                FxValueBox {
                    enabled: stereoEnhToggle.isOn
                    sliderValue: stereoSlider.controlValue
                }
                FxResetButton {
                    enabled: stereoEnhToggle.isOn
                    useNoArgReset: true
                    onResetNoArg: musicModel.reset_std_stereo_enhance()
                }
            }

            // HEADPHONE CROSSFEED
            RowLayout {
                Layout.fillWidth: true
                spacing: 4

                FxToggleBox {
                    id: crossfeedToggle
                    title: "CROSSFEED"
                    isOn: musicModel.crossfeed_active
                    onToggled: musicModel.toggleStdCrossfeed()
                }

                FxSliderBox {
                    id: crossfeedSlider
                    enabled: crossfeedToggle.isOn
                    controlValue: musicModel.crossfeed_amount
                    onSliderChanged: val => musicModel.setStdCrossfeedAmount(val)
                }
                FxValueBox {
                    enabled: crossfeedToggle.isOn
                    sliderValue: crossfeedSlider.controlValue
                }
                FxResetButton {
                    enabled: crossfeedToggle.isOn
                    useNoArgReset: true
                    onResetNoArg: musicModel.reset_std_crossfeed()
                }
            }

            // CRYSTALIZER - 3 box layout
            RowLayout {
                Layout.fillWidth: true
                spacing: 4

                FxToggleBox {
                    id: crystalToggle
                    title: "CRYSTALIZER"
                    isOn: musicModel.crystal_magic_active
                    onToggled: musicModel.toggleStdCrystalizer()
                }

                FxSliderBox {
                    id: crystalAmtSlider
                    enabled: crystalToggle.isOn
                    controlValue: musicModel.crystal_amount
                    onSliderChanged: val => musicModel.set_crystalizer_amount(val)
                }
                FxValueBox {
                    enabled: crystalToggle.isOn
                    sliderValue: crystalAmtSlider.controlValue
                }
                FxResetButton {
                    enabled: crystalToggle.isOn
                    useNoArgReset: true
                    onResetNoArg: musicModel.reset_std_crystalizer()
                }
            }

            // BASS BOOSTER - mode buttons with amount
            RowLayout {
                Layout.fillWidth: true
                spacing: 4

                FxToggleBox {
                    id: bassToggle
                    title: "BASS BOOSTER"
                    isOn: musicModel.bass_magic_active
                    onToggled: musicModel.toggleStdBassBooster()
                }

                BassModeSelector {
                    id: bassModeSelector
                    boxEnabled: bassToggle.isOn && musicModel.dsp_enabled
                    Layout.fillWidth: true
                    onModeChanged: mode => {
                        var freqs = [50, 60, 90, 150]
                        musicModel.setStdBassCutoff(freqs[mode])
                    }
                }

                FxBassAmountBox {
                    boxEnabled: bassToggle.isOn && musicModel.dsp_enabled
                    currentValue: musicModel.bass_gain
                    onValueChanged: val => musicModel.setStdBassGain(val)
                }

                FxResetButton {
                    enabled: bassToggle.isOn && musicModel.dsp_enabled
                    useNoArgReset: true
                    onResetNoArg: musicModel.reset_std_bass()
                }
            }

            // PITCH SHIFTER
            RowLayout {
                Layout.fillWidth: true
                spacing: 4

                FxToggleBox {
                    id: pitchToggle
                    title: "PITCH SHIFTER"
                    isOn: musicModel.pitch_active
                    onToggled: !musicModel.dsp_enabled ? {} : musicModel.toggleStdPitch()
                }

                FxPitchSliderBox {
                    id: pitchSlider
                    enabled: pitchToggle.isOn
                    controlValue: musicModel.pitch_semitones
                    onSliderChanged: val => musicModel.setStdPitchSemitones(val)
                }
                FxValueBox {
                    enabled: pitchToggle.isOn
                    sliderValue: pitchSlider.controlValue
                    showSemitones: true
                }
                FxResetButton {
                    enabled: pitchToggle.isOn
                    defaultValue: 0.0
                    sliderValue: pitchSlider.controlValue
                    onReset: val => musicModel.setStdPitchSemitones(val)
                }
            }
        }
    }

    // Toggle box - name with toggle at beginning
    component FxToggleBox: Rectangle {
        id: rootItem
        property string title: ""
        property bool isOn: false
        signal toggled

        Layout.fillWidth: false
        Layout.preferredWidth: 150
        Layout.preferredHeight: 24
        color: theme.colormap.fxgridbg || theme.colormap.fxbg
        radius: 4
        antialiasing: false

        RowLayout {
            anchors.fill: parent
            anchors.leftMargin: 4
            anchors.rightMargin: 4
            spacing: 0

            Text {
                text: isOn ? '󰔡' : '󰨙'
                font.family: symbols.name
                font.pixelSize: 16
                color: isOn ? theme.colormap.fxhover : theme.colormap.fxsubtext
                Layout.preferredWidth: 30
                MouseArea {
                    id: toggleIconArea
                    anchors.fill: parent
                    onClicked: rootItem.toggled()
                }
            }

            Text {
                text: title
                font.family: kodeMono.name
                font.pixelSize: 11
                color: isOn ? theme.colormap.fxtext : theme.colormap.fxsubtext
                Layout.preferredWidth: 160
                elide: Text.ElideRight
                MouseArea {
                    anchors.fill: parent
                    onClicked: rootItem.toggled()
                }
            }
        }
    }


    // Slider content - label + slider only
    component FxSliderBox: Rectangle {
        id: rootItem
        property real controlValue: 0.0
        property real currentValue: controlValue
        property string leftLabel: ""
        signal sliderChanged(real val)

        onControlValueChanged: {
            if (sld && !sld.pressed) {
                sld.value = controlValue;
                rootItem.currentValue = controlValue;
            }
        }

        Layout.fillWidth: true
        Layout.preferredHeight: 24
        color: theme.colormap.fxgridbg || theme.colormap.fxbg
        radius: 4
        antialiasing: false

        RowLayout {
            anchors.fill: parent
            anchors.leftMargin: 6
            anchors.rightMargin: 6
            spacing: 6

            Text {
                text: leftLabel
                font.family: kodeMono.name
                font.pixelSize: 11
                color: theme.colormap.fxsubtext
                visible: leftLabel !== ""
            }

            Slider {
                id: sld
                Layout.fillWidth: true
                Layout.fillHeight: true
                from: 0.0
                to: 1.0
                stepSize: 0.01
                value: rootItem.controlValue
                onValueChanged: rootItem.currentValue = sld.value
                onMoved: rootItem.sliderChanged(sld.value)

                WheelHandler {
                    target: sld
                    acceptedDevices: PointerDevice.Mouse | PointerDevice.TouchPad
                    orientation: Qt.Vertical
                    onWheel: function(event) {
                        var step = 0.05;
                        var delta = event.angleDelta.y > 0 ? step : -step;
                        var newVal = Math.max(0.0, Math.min(1.0, sld.value + delta));
                        sld.value = newVal;
                        rootItem.sliderChanged(newVal);
                    }
                }

                background: Rectangle {
                    x: sld.leftPadding
                    y: sld.topPadding + sld.availableHeight / 2 - height / 2
                    width: sld.availableWidth
                    height: 4
                    radius: 2
                    color: theme.colormap.fxsliderbg
                    Rectangle {
                        width: sld.visualPosition * parent.width
                        height: 4
                        radius: 2
                        color: theme.colormap.fxslider
                    }
                }
                handle: Rectangle {
                    x: sld.leftPadding + sld.visualPosition * (sld.availableWidth - 10)
                    y: sld.topPadding + sld.availableHeight / 2 - 5
                    width: 10
                    height: 10
                    radius: 5
                    color: theme.colormap.fxhandle
                }
            }
        }
    }

    // Slider with value combined - 4 box layout
    component FxSliderValueBox: Rectangle {
        id: rootItem
        property real controlValue: 0.0
        property real currentValue: controlValue
        property bool showHz: false
        property bool showKhz: false
        property real hzMin: 0.0
        property real hzMax: 10000.0
        signal sliderChanged(real val)

        onControlValueChanged: {
            if (svdSld && !svdSld.pressed) {
                svdSld.value = controlValue;
                rootItem.currentValue = controlValue;
            }
        }

        Layout.fillWidth: true
        Layout.preferredHeight: 24
        color: theme.colormap.fxgridbg || theme.colormap.fxbg
        radius: 4
        antialiasing: false

        RowLayout {
            anchors.fill: parent
            anchors.leftMargin: 6
            anchors.rightMargin: 6
            spacing: 4

            Slider {
                id: svdSld
                Layout.fillWidth: true
                Layout.fillHeight: true
                from: 0.0
                to: 1.0
                stepSize: 0.01
                value: rootItem.controlValue
                onValueChanged: rootItem.currentValue = svdSld.value
                onMoved: rootItem.sliderChanged(svdSld.value)

                WheelHandler {
                    target: svdSld
                    acceptedDevices: PointerDevice.Mouse | PointerDevice.TouchPad
                    orientation: Qt.Vertical
                    onWheel: function(event) {
                        var step = 0.05;
                        var delta = event.angleDelta.y > 0 ? step : -step;
                        var newVal = Math.max(0.0, Math.min(1.0, svdSld.value + delta));
                        svdSld.value = newVal;
                        rootItem.sliderChanged(newVal);
                    }
                }

                background: Rectangle {
                    x: svdSld.leftPadding
                    y: svdSld.topPadding + svdSld.availableHeight / 2 - height / 2
                    width: svdSld.availableWidth
                    height: 4
                    radius: 2
                    color: theme.colormap.fxsliderbg
                    Rectangle {
                        width: svdSld.visualPosition * parent.width
                        height: 4
                        radius: 2
                        color: theme.colormap.fxslider
                    }
                }
                handle: Rectangle {
                    x: svdSld.leftPadding + svdSld.visualPosition * (svdSld.availableWidth - 10)
                    y: svdSld.topPadding + svdSld.availableHeight / 2 - 5
                    width: 10
                    height: 10
                    radius: 5
                    color: theme.colormap.fxhandle
                }
            }

            Text {
                text: {
                    if (showHz) {
                        var freq = hzMin + (controlValue * (hzMax - hzMin));
                        return Math.round(freq) + " Hz";
                    } else if (showKhz) {
                        var freq = (hzMin + (controlValue * (hzMax - hzMin))) / 1000;
                        return freq.toFixed(1) + " kHz";
                    } else {
                        return Math.round(controlValue * 100) + "%";
                    }
                }
                font.family: sansSerif.name
                font.pixelSize: 11
                color: theme.colormap.fxsubtext
                Layout.preferredWidth: 60
            }
        }
    }

    // Bass mode button - just label
    component FxBassModeButton: Rectangle {
        id: rootItem
        property string modeLabel: ""
        property bool isActive: false
        signal clicked()

        Layout.fillWidth: true
        Layout.preferredHeight: 24
        color: theme.colormap.fxgridbg || theme.colormap.fxbg
        radius: 4
        antialiasing: false

        Text {
            anchors.centerIn: parent
            text: modeLabel
            font.family: kodeMono.name
            font.pixelSize: 11
            font.bold: isActive
            color: isActive ? theme.colormap.fxtext : theme.colormap.fxsubtext
        }

        MouseArea {
            anchors.fill: parent
            onClicked: rootItem.clicked()
        }
    }

    // Bass mode selector with state
    component BassModeSelector: Item {
        id: bassModeRoot
        property int selectedMode: 2
        property bool boxEnabled: true
        signal modeChanged(int mode)

        Layout.fillWidth: true
        Layout.preferredHeight: 24

        RowLayout {
            anchors.fill: parent
            spacing: 2
            enabled: bassModeRoot.boxEnabled

            FxBassModeButton {
                modeLabel: "Deep"
                isActive: bassModeRoot.selectedMode === 0
                onClicked: {
                    bassModeRoot.selectedMode = 0
                    bassModeRoot.modeChanged(0)
                }
            }
            FxBassModeButton {
                modeLabel: "Soft"
                isActive: bassModeRoot.selectedMode === 1
                onClicked: {
                    bassModeRoot.selectedMode = 1
                    bassModeRoot.modeChanged(1)
                }
            }
            FxBassModeButton {
                modeLabel: "Punch"
                isActive: bassModeRoot.selectedMode === 2
                onClicked: {
                    bassModeRoot.selectedMode = 2
                    bassModeRoot.modeChanged(2)
                }
            }
            FxBassModeButton {
                modeLabel: "Warm"
                isActive: bassModeRoot.selectedMode === 3
                onClicked: {
                    bassModeRoot.selectedMode = 3
                    bassModeRoot.modeChanged(3)
                }
            }
        }
    }

    // Editable amount box for bass
    component FxBassAmountBox: Rectangle {
        id: rootItem
        property real currentValue: 0.0
        property real minValue: 0.0
        property real maxValue: 12.0
        property bool boxEnabled: true
        signal valueChanged(real val)

        Layout.preferredWidth: 60
        Layout.preferredHeight: 24
        color: theme.colormap.fxgridbg || theme.colormap.fxbg
        radius: 4
        antialiasing: false
        opacity: boxEnabled ? 1.0 : 0.5

        state: "display"

        Text {
            id: displayText
            anchors.centerIn: parent
            text: Math.round(rootItem.currentValue / rootItem.maxValue * 100) + "%"
            font.family: sansSerif.name
            font.pixelSize: 11
            color: theme.colormap.fxsubtext
            visible: rootItem.state === "display"
        }

        TextInput {
            id: inputField
            anchors.centerIn: parent
            width: 35
            horizontalAlignment: TextInput.AlignHCenter
            font.family: sansSerif.name
            font.pixelSize: 11
            color: theme.colormap.fxtext
            visible: rootItem.state === "edit"
            validator: IntValidator { bottom: 0; top: 100 }
            onAccepted: {
                var val = parseInt(text)
                if (!isNaN(val)) {
                    val = Math.max(0, Math.min(100, val))
                    rootItem.currentValue = val / 100 * rootItem.maxValue
                    rootItem.valueChanged(rootItem.currentValue)
                }
                rootItem.state = "display"
            }
            onActiveFocusChanged: {
                if (!activeFocus) {
                    rootItem.state = "display"
                }
            }
        }

        MouseArea {
            id: hoverArea
            anchors.fill: parent
            hoverEnabled: true
            onEntered: displayText.color = theme.colormap.fxtext
            onExited: displayText.color = theme.colormap.fxsubtext
            onClicked: rootItem.state = "display"
            onDoubleClicked: {
                inputField.text = Math.round(rootItem.currentValue / rootItem.maxValue * 100)
                rootItem.state = "edit"
                inputField.forceActiveFocus()
                inputField.selectAll()
            }
            onWheel: event => {
                var delta = event.angleDelta.y > 0 ? 0.5 : -0.5
                var newVal = Math.max(rootItem.minValue, Math.min(rootItem.maxValue, rootItem.currentValue + delta))
                rootItem.currentValue = newVal
                rootItem.valueChanged(newVal)
            }
        }
    }

    // Dual value box: "X% | YkHz"
    component FxValueBox2: Rectangle {
        id: rootItem
        property real percentValue: 0.0
        property real freqValue: 0.0
        property real hzMin: 0.0
        property real hzMax: 10000.0
        property bool showKhz: false

        Layout.preferredWidth: 60
        Layout.preferredHeight: 24
        color: theme.colormap.fxgridbg || theme.colormap.fxbg
        radius: 4
        antialiasing: false

        Text {
            anchors.centerIn: parent
            text: {
                var pct = Math.round(percentValue * 100) + "%";
                var freq = hzMin + (freqValue * (hzMax - hzMin));
                if (showKhz) {
                    freq = (freq / 1000).toFixed(1) + " kHz";
                } else {
                    freq = Math.round(freq) + " Hz";
                }
                return pct + " | " + freq;
            }
            font.family: sansSerif.name
            font.pixelSize: 10
            color: theme.colormap.fxsubtext
        }
    }

    // Value display box
    component FxValueBox: Rectangle {
        id: rootItem
        property real sliderValue: 0.0
        property bool showHz: false
        property real hzMin: 0.0
        property real hzMax: 10000.0
        property bool showSemitones: false

        Layout.preferredWidth: 60
        Layout.preferredHeight: 24
        color: theme.colormap.fxgridbg || theme.colormap.fxbg
        radius: 4
        antialiasing: false

        Text {
            anchors.centerIn: parent
            text: {
                if (showHz) {
                    var freq = hzMin + (sliderValue * (hzMax - hzMin));
                    return Math.round(freq) + " Hz";
                } else if (showSemitones) {
                    if (sliderValue === 0) return "0 ST";
                    return (sliderValue > 0 ? "+" : "") + Math.round(sliderValue) + " ST";
                } else {
                    return Math.round(sliderValue * 100) + "%";
                }
            }
            font.family: sansSerif.name
            font.pixelSize: 11
            color: theme.colormap.fxsubtext
        }
    }

    // Reset button box
    component FxResetButton: Rectangle {
        id: rootItem
        property real defaultValue: 0.0
        property real sliderValue: 0.0
        property bool showHz: false
        property real hzMin: 0.0
        property real hzMax: 10000.0
        property bool useNoArgReset: false
        signal reset(real val)
        signal resetNoArg()

        Layout.preferredWidth: 24
        Layout.preferredHeight: 24
        color: theme.colormap.fxgridbg || theme.colormap.fxbg
        radius: 4
        antialiasing: false

        Text {
            anchors.centerIn: parent
            text: '󰜉'
            font.family: symbols.name
            font.pixelSize: 12
            color: theme.colormap.fxsubtext
        }

        MouseArea {
            anchors.fill: parent
            onClicked: {
                if (rootItem.useNoArgReset) {
                    rootItem.resetNoArg();
                } else {
                    var resetVal = rootItem.defaultValue;
                    rootItem.reset(resetVal);
                }
            }
        }
    }

    // Pitch slider box - special with center marker
    component FxPitchSliderBox: Rectangle {
        id: rootItem
        property real controlValue: 0.0
        property real currentValue: controlValue
        signal sliderChanged(real val)

        onControlValueChanged: {
            if (pitchSld && !pitchSld.pressed) {
                pitchSld.value = controlValue;
                rootItem.currentValue = controlValue;
            }
        }

        Layout.fillWidth: true
        Layout.preferredHeight: 24
        color: theme.colormap.fxgridbg || theme.colormap.fxbg
        radius: 4
        antialiasing: false

        Slider {
            id: pitchSld
            anchors.fill: parent
            anchors.margins: 6
            from: -12.0
            to: 12.0
            stepSize: 1.0
            value: rootItem.controlValue
            onValueChanged: rootItem.currentValue = pitchSld.value
            onMoved: {
                var v = pitchSld.value;
                if (Math.abs(v) < 0.5) v = 0.0;
                rootItem.sliderChanged(v);
            }

            WheelHandler {
                target: pitchSld
                acceptedDevices: PointerDevice.Mouse | PointerDevice.TouchPad
                orientation: Qt.Vertical
                onWheel: function(event) {
                    var step = 1.0;
                    var delta = event.angleDelta.y > 0 ? step : -step;
                    var newVal = Math.max(-12.0, Math.min(12.0, pitchSld.value + delta));
                    if (Math.abs(newVal) < 0.5) newVal = 0.0;
                    pitchSld.value = newVal;
                    rootItem.sliderChanged(newVal);
                }
            }

            background: Rectangle {
                x: pitchSld.leftPadding
                y: pitchSld.topPadding + pitchSld.availableHeight / 2 - height / 2
                width: pitchSld.availableWidth
                height: 4
                radius: 2
                color: theme.colormap.fxsliderbg

                Rectangle {
                    width: 2
                    height: 8
                    anchors.centerIn: parent
                    color: theme.colormap.fxsubtext
                    opacity: 0.5
                }

                Rectangle {
                    anchors.verticalCenter: parent.verticalCenter
                    height: 4
                    radius: 2
                    color: theme.colormap.fxslider
                    x: pitchSld.visualPosition >= 0.5 ? parent.width / 2 : pitchSld.visualPosition * parent.width
                    width: Math.abs(pitchSld.visualPosition - 0.5) * parent.width
                }
            }
            handle: Rectangle {
                x: pitchSld.leftPadding + pitchSld.visualPosition * (pitchSld.availableWidth - 10)
                y: pitchSld.topPadding + pitchSld.availableHeight / 2 - 5
                width: 10
                height: 10
                radius: 5
                color: theme.colormap.fxhandle
            }
        }
    }
}
