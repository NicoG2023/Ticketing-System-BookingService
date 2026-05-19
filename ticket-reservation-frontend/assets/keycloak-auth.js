let keycloak = null;
let KeycloakConstructor = null;
let importPromise = null;
let initPromise = null;

console.log("[keycloak-auth] Script cargado");

async function loadKeycloakConstructor() {
  if (KeycloakConstructor) {
    return KeycloakConstructor;
  }

  if (!importPromise) {
    importPromise = import("./vendor/keycloak/keycloak.js");
  }

  const module = await importPromise;

  KeycloakConstructor = module.default || module.Keycloak || module;

  if (!KeycloakConstructor) {
    throw new Error("No se pudo cargar el constructor de Keycloak");
  }

  return KeycloakConstructor;
}

window.initKeycloak = async function () {
  console.log("[keycloak-auth] initKeycloak llamado");

  const Keycloak = await loadKeycloakConstructor();

  if (keycloak) {
    return keycloak.authenticated === true;
  }

  keycloak = new Keycloak({
    url: "http://localhost:8092",
    realm: "ticket-reservation",
    clientId: "ticket-frontend",
  });

  if (!initPromise) {
    initPromise = keycloak.init({
      pkceMethod: "S256",
      checkLoginIframe: false,
    });
  }

  const authenticated = await initPromise;

  return authenticated === true;
};

window.login = async function () {
  console.log("[keycloak-auth] login llamado");

  if (!keycloak) {
    await window.initKeycloak();
  }

  await keycloak.login({
    redirectUri: window.location.origin + "/",
  });

  return true;
};

window.logout = async function () {
  console.log("[keycloak-auth] logout llamado");

  if (!keycloak) {
    await window.initKeycloak();
  }

  await keycloak.logout({
    redirectUri: window.location.origin + "/",
  });

  return true;
};

window.getToken = async function () {
  if (!keycloak) {
    await window.initKeycloak();
  }

  if (!keycloak || !keycloak.authenticated) {
    return null;
  }

  await keycloak.updateToken(30);

  return keycloak.token ?? null;
};

window.isAuthenticated = function () {
  return keycloak?.authenticated === true;
};

window.getUsername = function () {
  return (
    keycloak?.tokenParsed?.preferred_username ||
    keycloak?.tokenParsed?.name ||
    null
  );
};

window.getUserId = function () {
  return keycloak?.tokenParsed?.sub || null;
};
