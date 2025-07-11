use mvengine_proc::r;

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
            <dimension name="ui_widget_height" val="10.5cm"/>
        </dimensions>
        <shapes>
            <shape name="tick" src="shapes/tick.msfx" language="MSFX"/>
        </shapes>
        <textures>
            <texture name="terrain_sand" src="textures/terrain/sand.png"/>
            <texture name="terrain_grass" src="textures/terrain/grass.png" sampler="linear"/>
            <texture name="terrain_stone" src="textures/terrain/stone.png"/>
            <texture name="terrain_water" src="textures/terrain/water.png"/>

            <texture name="tile_wood" src="textures/tiles/wood.png"/>
            <texture name="tile_generator" src="textures/tiles/generator.png"/>

            <texture name="player" src="textures/player.png"/>
            <texture name="bg" src="textures/bg.png"/>
        </textures>
        <adaptives>

        </adaptives>
        <fonts>
            <font name="default" src="fonts/data.font" atlas="fonts/atlas.png"/>
        </fonts>
        <tilesets>
            <tileset name="lamp" atlas="textures/tiles/lamp.png" width="64" height="64" count="2">
                <entry name="on" index="0"/>
                <entry name="off" index="1"/>
            </tileset>
        </tilesets>
        <animations>
            
        </animations>
        <drawables>
            <drawable name="bg" type="texture" ref="bg"/>
        </drawables>
        <geometries>
            <geometry name="tick" type="shape" ref="tick"/>
        </geometries>
    </resources>
}