use mvengine::ui::context::UiResources;
use mvengine_proc::r;

r! {
    <resources structName="R" cdir="../resources/">
        <shapes>

        </shapes>
        <textures>
            <texture name="tile_grass" src="textures/tiles/grass.png"/>
            <texture name="tile_sand" src="textures/tiles/sand.png"/>
            <texture name="tile_stone" src="textures/tiles/stone.png"/>
            <texture name="tile_water" src="textures/tiles/water.png"/>
            <texture name="machine_bore" src="textures/machines/bore.png"/>
            <texture name="player" src="textures/player.png"/>
        </textures>
        <fonts>
            <font name="default" src="fonts/data.font" atlas="fonts/atlas.png"/>
        </fonts>
    </resources>
}