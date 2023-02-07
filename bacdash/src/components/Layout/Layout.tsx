import {
    Nav,
    NavList,
    Page,
    PageHeader,
    PageSidebar,
} from "@patternfly/react-core";
import React from "react";

interface ILayout {
    children: React.ReactNode;
}

export const AppLayout: React.FunctionComponent<ILayout> = ({ children }) => {
    const [isNavOpen, setIsNavOpen] = React.useState(true);
    const [isMobileView, setIsMobileView] = React.useState(false);
    const [isNavOpenMobile, setIsNavOpenMobile] = React.useState(false);

    const onNavToggleMobile = () => setIsNavOpenMobile(!isNavOpenMobile);
    const onNavToggle = () => setIsNavOpen(!isNavOpen);

    const onPageResize = (props: { mobileView: boolean; windowSize: number }) => {
        setIsMobileView(props.mobileView);
    };

    const Header = (
        <PageHeader
            logo={<p>Bacman</p>}
            showNavToggle
            isNavOpen={isNavOpen}
            onNavToggle={isMobileView ? onNavToggleMobile : onNavToggle}
        />
    );

    const Navigation = (
        <Nav id="nav-primary-simple" theme="dark">
            <NavList id="nav-list-simple"></NavList>
        </Nav>
    );

    const Sidebar = (
        <PageSidebar
            theme="dark"
            nav={Navigation}
            isNavOpen={isMobileView ? isNavOpenMobile : isNavOpen}
        />
    );

    const pageId = "primary-app-container";

    return <Page header={Header} sidebar={Sidebar}>{children}</Page>;
};
