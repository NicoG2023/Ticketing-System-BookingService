use wasm_bindgen::prelude::*;

#[wasm_bindgen(inline_js = r#"
let keycloak = null;
let KeycloakConstructor = null;
let importPromise = null;
let initPromise = null;
let keycloakJsUrl = null;

let keycloakConfig = {
  url: "http://localhost:8092",
  realm: "ticket-reservation",
  clientId: "ticket-frontend",
};

function getRedirectUri() {
  return window.location.origin + "/";
}

export function setKeycloakJsUrl(url) {
  keycloakJsUrl = url;
  console.log("[keycloak-auth] Keycloak JS URL:", keycloakJsUrl);
}

export function configureKeycloak(url, realm, clientId) {
  keycloakConfig = {
    url,
    realm,
    clientId,
  };

  console.log("[keycloak-auth] Configuración Keycloak:", keycloakConfig);
}

async function loadKeycloakConstructor() {
  if (KeycloakConstructor) {
    return KeycloakConstructor;
  }

  if (!keycloakJsUrl) {
    throw new Error("No se configuró la URL del archivo keycloak.js");
  }

  if (!importPromise) {
    importPromise = import(keycloakJsUrl);
  }

  const module = await importPromise;

  KeycloakConstructor = module.default || module.Keycloak || module;

  if (!KeycloakConstructor) {
    throw new Error("No se pudo cargar el constructor de Keycloak");
  }

  return KeycloakConstructor;
}

async function ensureKeycloakInitialized() {
  if (!keycloak) {
    await initKeycloak();
  }

  if (!keycloak) {
    throw new Error("No se pudo inicializar Keycloak");
  }

  return keycloak;
}

export async function initKeycloak() {
  console.log("[keycloak-auth] initKeycloak llamado");
  console.log("[keycloak-auth] href actual:", window.location.href);

  const Keycloak = await loadKeycloakConstructor();

  if (keycloak) {
    const alreadyAuthenticated = keycloak.authenticated === true;
    console.log("[keycloak-auth] keycloak ya existe. authenticated:", alreadyAuthenticated);
    return alreadyAuthenticated;
  }

  keycloak = new Keycloak({
    url: keycloakConfig.url,
    realm: keycloakConfig.realm,
    clientId: keycloakConfig.clientId,
  });

  if (!initPromise) {
    initPromise = keycloak.init({
      pkceMethod: "S256",
      checkLoginIframe: false,
      responseMode: "query",
      flow: "standard",
      redirectUri: getRedirectUri(),
    });
  }

  const authenticated = await initPromise;

  console.log("[keycloak-auth] init result authenticated:", authenticated);
  console.log("[keycloak-auth] token existe:", Boolean(keycloak.token));
  console.log(
    "[keycloak-auth] username:",
    keycloak?.tokenParsed?.preferred_username ||
      keycloak?.tokenParsed?.name ||
      "null"
  );

  return authenticated === true;
}

export async function login() {
  console.log("[keycloak-auth] login llamado");

  const kc = await ensureKeycloakInitialized();

  const redirectUri = getRedirectUri();

  console.log("[keycloak-auth] redirectUri usado:", redirectUri);

  const loginUrl = await kc.createLoginUrl({
    redirectUri,
  });

  console.log("[keycloak-auth] loginUrl:", loginUrl);
  console.log("[keycloak-auth] navegando a loginUrl");

  window.location.assign(loginUrl);

  return true;
}

export async function logout() {
  console.log("[keycloak-auth] logout llamado");

  const kc = await ensureKeycloakInitialized();

  if (!kc.authenticated) {
    console.log("[keycloak-auth] logout ignorado: no hay sesión autenticada");
    return true;
  }

  await kc.logout({
    redirectUri: getRedirectUri(),
  });

  return true;
}

export async function getToken() {
  const kc = await ensureKeycloakInitialized();

  if (!kc.authenticated) {
    return null;
  }

  try {
    await kc.updateToken(30);
  } catch (error) {
    console.error("[keycloak-auth] error actualizando token:", error);
    return null;
  }

  return kc.token ?? null;
}

export function isAuthenticated() {
  return keycloak?.authenticated === true;
}

export function getUsername() {
  return (
    keycloak?.tokenParsed?.preferred_username ||
    keycloak?.tokenParsed?.name ||
    null
  );
}

export function getUserId() {
  return keycloak?.tokenParsed?.sub || null;
}

export function getParsedToken() {
  return keycloak?.tokenParsed || null;
}
"#)]
extern "C" {
    #[wasm_bindgen(js_name = setKeycloakJsUrl)]
    fn js_set_keycloak_js_url(url: &str);

    #[wasm_bindgen(js_name = configureKeycloak)]
    fn js_configure_keycloak(url: &str, realm: &str, client_id: &str);

    #[wasm_bindgen(catch, js_name = initKeycloak)]
    async fn js_init_keycloak() -> Result<JsValue, JsValue>;

    #[wasm_bindgen(catch, js_name = login)]
    async fn js_login() -> Result<JsValue, JsValue>;

    #[wasm_bindgen(catch, js_name = logout)]
    async fn js_logout() -> Result<JsValue, JsValue>;

    #[wasm_bindgen(catch, js_name = getToken)]
    async fn js_get_token() -> Result<JsValue, JsValue>;

    #[wasm_bindgen(catch, js_name = isAuthenticated)]
    fn js_is_authenticated() -> Result<bool, JsValue>;

    #[wasm_bindgen(catch, js_name = getUsername)]
    fn js_get_username() -> Result<JsValue, JsValue>;

    #[wasm_bindgen(catch, js_name = getUserId)]
    fn js_get_user_id() -> Result<JsValue, JsValue>;
}

pub fn set_keycloak_js_url(url: &str) {
    js_set_keycloak_js_url(url);
}

pub fn configure_keycloak(url: &str, realm: &str, client_id: &str) {
    js_configure_keycloak(url, realm, client_id);
}

pub async fn init_keycloak() -> Result<bool, String> {
    js_init_keycloak()
        .await
        .map_err(js_error_to_string)?
        .as_bool()
        .ok_or_else(|| "Keycloak no retornó un booleano válido".to_string())
}

pub async fn login() -> Result<(), String> {
    js_login().await.map(|_| ()).map_err(js_error_to_string)
}

pub async fn logout() -> Result<(), String> {
    js_logout().await.map(|_| ()).map_err(js_error_to_string)
}

pub async fn get_token() -> Result<Option<String>, String> {
    js_get_token()
        .await
        .map(|value| value.as_string())
        .map_err(js_error_to_string)
}

pub fn is_authenticated() -> Result<bool, String> {
    js_is_authenticated().map_err(js_error_to_string)
}

pub fn get_username() -> Option<String> {
    js_get_username().ok().and_then(|value| value.as_string())
}

pub fn get_user_id() -> Option<String> {
    js_get_user_id().ok().and_then(|value| value.as_string())
}

fn js_error_to_string(error: JsValue) -> String {
    if let Some(message) = error.as_string() {
        return message;
    }

    if let Ok(message) = js_sys::Reflect::get(&error, &JsValue::from_str("message")) {
        if let Some(message) = message.as_string() {
            return message;
        }
    }

    if let Ok(name) = js_sys::Reflect::get(&error, &JsValue::from_str("name")) {
        if let Some(name) = name.as_string() {
            return format!("Error JavaScript de Keycloak: {name}");
        }
    }

    "Error ejecutando función JavaScript de Keycloak".to_string()
}
