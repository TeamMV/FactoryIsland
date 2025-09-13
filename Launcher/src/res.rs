use mvengine_proc::r;

use mvengine::ui::context::UiResources;

r! {
    <resources structName="R" cdir="../res/">
        <colors>
            <color name="inv_bg" val="white"/>
            <color name="inv_bg_border" val="black"/>
            <color name="inv_slot" val="white"/>
            <color name="inv_slot_select" val="red"/>
            <color name="inv_slot_border" val="black"/>

            <color name="ui_bg" val="#363636AA"/>
            <color name="ui_highlight" val="#EDD605FF"/>
        </colors>
        <dimensions>
            <dimension name="ui_widget_width" val="10cm"/>
            <dimension name="ui_widget_height" val="1.5cm"/>
        </dimensions>
        <shapes>
            <shape name="tick" src="shapes/tick.msfx" language="MSFX"/>
            <shape name="knob" src="shapes/knob.msfx" language="MSFX"/>
        </shapes>
        <textures>

        </textures>
        <adaptives>

        </adaptives>
        <fonts>
            <font name="default" src="fonts/data.font" atlas="fonts/atlas.png"/>
        </fonts>
        <tilesets>

        </tilesets>
        <animations>

        </animations>
        <drawables>

        </drawables>
        <geometries>
            <geometry name="tick" type="shape" ref="tick"/>
            <geometry name="knob" type="shape" ref="knob"/>
        </geometries>
    </resources>
}