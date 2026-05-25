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
  return window.location.origin + window.location.pathname;
}

function getSilentCheckSsoUri() {
  return window.location.origin + "/silent-check-sso.html";
}

function notifySessionExpired() {
  window.dispatchEvent(
    new CustomEvent("ticket-auth-session-expired", {
      detail: {
        message: "Tu sesión expiró. Inicia sesión de nuevo para continuar.",
      },
    }),
  );
}

let refreshPromise = null;

async function refreshTokenOrNotify() {
  if (!keycloak) {
    notifySessionExpired();
    return null;
  }

  if (!keycloak.authenticated) {
    notifySessionExpired();
    return null;
  }

  try {
    if (!refreshPromise) {
      refreshPromise = keycloak.updateToken(30);
    }

    const refreshed = await refreshPromise;

    console.log("[keycloak-auth] token refresh result:", refreshed);

    refreshPromise = null;

    return keycloak.token ?? null;
  } catch (error) {
    refreshPromise = null;

    console.error("[keycloak-auth] refresh token falló:", error);

    notifySessionExpired();

    return null;
  }
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
    return keycloak.authenticated === true;
  }

  keycloak = new Keycloak({
    url: keycloakConfig.url,
    realm: keycloakConfig.realm,
    clientId: keycloakConfig.clientId,
  });

  keycloak.onTokenExpired = () => {
    console.warn("[keycloak-auth] access token expirado, intentando refrescar...");

    refreshTokenOrNotify();
  };

  keycloak.onAuthRefreshError = () => {
    console.warn("[keycloak-auth] no fue posible refrescar el token");
    notifySessionExpired();
  };

  keycloak.onAuthLogout = () => {
    console.warn("[keycloak-auth] sesión cerrada");
  };

  if (!initPromise) {
    initPromise = keycloak.init({
      onLoad: "check-sso",
      silentCheckSsoRedirectUri: getSilentCheckSsoUri(),

      pkceMethod: "S256",
      checkLoginIframe: false,
      flow: "standard",

      // Yo quitaría responseMode: "query" salvo que tengas una razón fuerte.
      // El default de Keycloak es fragment.
      // responseMode: "fragment",
    });
  }

  const authenticated = await initPromise;

  console.log("[keycloak-auth] init result authenticated:", authenticated);
  console.log("[keycloak-auth] token existe:", Boolean(keycloak.token));
  console.log(
    "[keycloak-auth] roles cliente:",
    keycloak?.tokenParsed?.resource_access?.[keycloakConfig.clientId]?.roles || [],
  );

  return authenticated === true;
}

export async function login() {
  console.log("[keycloak-auth] login llamado");

  const kc = await ensureKeycloakInitialized();

  const redirectUri = window.location.href;

  const loginUrl = await kc.createLoginUrl({
    redirectUri,
  });

  window.location.assign(loginUrl);

  return true;
}

export async function logout() {
  console.log("[keycloak-auth] logout llamado");

  const kc = await ensureKeycloakInitialized();

  if (!kc.authenticated) {
    return true;
  }

  await kc.logout({
    redirectUri: window.location.origin + "/",
  });

  return true;
}

export async function getToken() {
  const kc = await ensureKeycloakInitialized();

  if (!kc.authenticated) {
    notifySessionExpired();
    return null;
  }

  return await refreshTokenOrNotify();
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

export function getClientRoles() {
  const clientId = keycloakConfig.clientId;

  const roles =
    keycloak?.tokenParsed?.resource_access?.[clientId]?.roles || [];

  return JSON.stringify(roles);
}

export function hasClientRole(role) {
  const clientId = keycloakConfig.clientId;

  const roles =
    keycloak?.tokenParsed?.resource_access?.[clientId]?.roles || [];

  return roles.includes(role);
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

    #[wasm_bindgen(catch, js_name = getClientRoles)]
    fn js_get_client_roles() -> Result<JsValue, JsValue>;

    #[wasm_bindgen(catch, js_name = hasClientRole)]
    fn js_has_client_role(role: &str) -> Result<bool, JsValue>;
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

pub fn get_client_roles() -> Vec<String> {
    let Some(roles_json) = js_get_client_roles()
        .ok()
        .and_then(|value| value.as_string())
    else {
        return Vec::new();
    };

    serde_json::from_str::<Vec<String>>(&roles_json).unwrap_or_default()
}

pub fn has_client_role(role: &str) -> bool {
    js_has_client_role(role).unwrap_or(false)
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
