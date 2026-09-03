//! Chronon app shell layout.

use lepton_shell::AppBarUserMenu;
use leptos::prelude::*;
use leptos_router::components::Outlet;
use orbital::components::{
    Navigation, NavigationBody, NavigationConfig, NavigationLink, NavigationMaterial,
};
use uf_integrations::{
    ShellAppBar, ShellAuthMenu, ShellLeftNav, UnifiedFieldAppBar, UnifiedFieldShellLayout,
};
use uf_product::routes::RequireAuthenticated;

use crate::paths;
use crate::AppMetadata;

/// Renders the standard Orbital shell for the Chronon app and nests page routes.
#[component]
pub fn ChrononLayout() -> impl IntoView {
    let app_name = AppMetadata::name().to_string();
    let selected_value = RwSignal::new(None::<String>);
    let open_categories = RwSignal::new(Vec::<String>::new());

    view! {
        <div data-testid="chronon-app-root" style="height: 100%;">
        <UnifiedFieldShellLayout>
            <ShellAppBar slot>
                <UnifiedFieldAppBar
                    app_name=app_name
                    app_id=AppMetadata::id()
                    homepage_url="/".to_string()
                >
                    <ShellAuthMenu slot:auth_menu>
                        <AppBarUserMenu />
                    </ShellAuthMenu>
                </UnifiedFieldAppBar>
            </ShellAppBar>
            <ShellLeftNav slot>
                <Navigation config=NavigationConfig::new().with_selected_value(selected_value).with_open_categories(open_categories)>
                    <NavigationMaterial slot />
                    <NavigationBody slot>
                        <div id="chronon-nav">
                            <NavigationLink path=paths::ROOT value=paths::ROOT icon=icondata::AiDashboardOutlined exact=true test_id="nav-dashboard">"Dashboard"</NavigationLink>
                            <NavigationLink path=paths::JOBS value=paths::JOBS icon=icondata::AiScheduleOutlined test_id="nav-jobs">"Jobs"</NavigationLink>
                            <NavigationLink path=paths::RUNS value=paths::RUNS icon=icondata::AiHistoryOutlined test_id="nav-runs">"Runs"</NavigationLink>
                            <NavigationLink path=paths::SCRIPTS value=paths::SCRIPTS icon=icondata::AiCodeOutlined test_id="nav-scripts">"Scripts"</NavigationLink>
                        </div>
                    </NavigationBody>
                </Navigation>
            </ShellLeftNav>
            <RequireAuthenticated>
                <Outlet />
            </RequireAuthenticated>
        </UnifiedFieldShellLayout>
        </div>
    }
}
