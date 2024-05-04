fn main(){
    println!("Con este codigo me da muchos errores, los cuales no se solventar. Leyendo los errores que me da durante la compilacion
    me aparecen cosas como que deberÍa usar una version rust superior a 2015 o incluso a 2018, y por lo que se, mi version
    es la mas reciente, por lo que no se que hacer");

    {
    &quot;id&quot; 1,
    &quot;name&quot;: &quot;maxwell&quot;,
    &quot;hp&quot;: 50,
    &quot;money&quot;: 99,
    &quot;score&quot;: 5,
    &quot;owner&quot;: {
    &quot;id&quot;: 1,
    &quot;username&quot;: &quot;admin&quot;,
    &quot;is_author&quot;: true
    }
    }
    const BASE_URL: &amp;str = &quot;http://127.0.0.1:8001/api/&quot;;
    async fn make_api_request(url: &amp;str) -&gt; Result&lt;serde_json::Value, Error&gt; {
    let resp = reqwest::get(url)
    .await?
    .json::&lt;serde_json::Value&gt;()
    .await?;
    Ok(resp)
    }
    pub async fn get_all_players() -&gt; Result&lt;Vec&lt;Player&gt;, ApiError&gt; {
    let url = &amp;format!(&quot;{}{}&quot;, BASE_URL, &quot;players/&quot;);
    match make_api_request(url).await {
    Ok(resp) =&gt; {
    match serde_json::from_value::&lt;Vec&lt;Player&gt;&gt;(resp) {
    Ok(players) =&gt; Ok(players),
    Err(e) =&gt; Err(ApiError::from(e)),
    }
    }
    Err(e) =&gt; Err(ApiError::from(e)),
    }
    }
    pub struct Player {
    hp: i32,
    id: i32,
    money: i32,
    name: String,
    score: i32,
    owner: Owner
    }
    
    async fn main() {
    match api::get_all_players().await {
    Ok(players) =&gt; {
    for player in players {
    println!(&quot;Jugador: {:?}&quot;, player);
    }
    }
    Err(e) =&gt; {
    eprintln!(&quot;Error al obtener los jugadores: {}&quot;, e);
    }
    }
    }
}