use mvengine::ui::context::UiResources;
use mvengine_proc::r;

r! {
    <resources structName="R" cdir="../resources/">
        <shapes>

        </shapes>
        <textures>
            <texture name="tile_grass" src="textures/tiles/grass.png" sampler="linear"/>
            <texture name="tile_sand" src="textures/tiles/sand.png"/>
            <texture name="tile_stone" src="textures/tiles/stone.png"/>
            <texture name="tile_water" src="textures/tiles/water.png"/>
            <texture name="nature_tree" src="textures/nature/tree1.png"/>
            <texture name="nature_cactus" src="textures/nature/cactus.png"/>
            <texture name="nature_mushroom" src="textures/nature/mushroom.png"/>
            <texture name="machine_bore" src="textures/machines/bore.png"/>
            <texture name="big_wall" src="textures/buildings/big_wall.png"/>
            <texture name="player" src="textures/player.png"/>
        </textures>
        <fonts>
            <font name="default" src="fonts/data.font" atlas="fonts/atlas.png"/>
        </fonts>
        <tilesets>
            <tileset name="bore" atlas="textures/machines/bore_anim.png" width="64" height="64" count="3">
                <entry name="disabled" index="0"/>
                <fps value="4"/>
            </tileset>
        </tilesets>
        <animations>
            <animation name="bore" tileset="bore" fps="4"/>
            <animation name="bore_fast" tileset="bore" fps="8"/>
            <animation name="bore_overloaded" tileset="bore" fps="24"/>
        </animations>
    </resources>
}