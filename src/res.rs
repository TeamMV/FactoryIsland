use mvengine_proc::r;

r! {
    <resources structName="R" cdir="../res/">
        <colors>
            <color name="inv_bg" val="white"/>
            <color name="inv_bg_border" val="black"/>
            <color name="inv_slot" val="white"/>
            <color name="inv_slot_select" val="red"/>
            <color name="inv_slot_border" val="black"/>
        </colors>
        <shapes>

        </shapes>
        <textures>
            <texture name="terrain_sand" src="textures/terrain/sand.png"/>
            <texture name="terrain_grass" src="textures/terrain/grass.png" sampler="linear"/>
            <texture name="terrain_stone" src="textures/terrain/stone.png"/>
            <texture name="terrain_water" src="textures/terrain/water.png"/>

            <texture name="tile_wood" src="textures/tiles/wood.png"/>
            <texture name="tile_generator" src="textures/tiles/generator.png"/>

            <texture name="player" src="textures/player.png"/>
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
    </resources>
}