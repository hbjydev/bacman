import { AppLayout } from "@app/components/Layout/Layout";
import {
    Breadcrumb,
    BreadcrumbItem,
    Card,
    CardBody,
    CardTitle,
    DescriptionList,
    DescriptionListDescription,
    DescriptionListGroup,
    DescriptionListTerm,
    Flex,
    FlexItem,
    Gallery,
    Grid,
    GridItem,
    Icon,
    PageBreadcrumb,
    PageSection,
    PageSectionVariants,
    Title,
} from "@patternfly/react-core";
import { CheckCircleIcon, DotCircleIcon, ExclamationCircleIcon } from '@patternfly/react-icons';
import { type NextPage } from "next";
import Head from "next/head";

const Home: NextPage = () => {
    return (
        <>
            <Head>
                <title>bacdash</title>
            </Head>

            <AppLayout>
                <PageBreadcrumb>
                    <Breadcrumb>
                        <BreadcrumbItem>Dashboard</BreadcrumbItem>
                    </Breadcrumb>
                </PageBreadcrumb>

                <PageSection variant={PageSectionVariants.light}>
                    <Title headingLevel="h1" size="2xl">
                        Dashboard
                    </Title>
                </PageSection>

                <PageSection>
                    <Grid hasGutter>

                        <GridItem span={3}>
                            <Card>
                                <CardTitle>Details</CardTitle>

                                <CardBody>
                                    <DescriptionList>
                                        <DescriptionListGroup>
                                            <DescriptionListTerm>Host</DescriptionListTerm>
                                            <DescriptionListDescription>127.0.0.1:6003</DescriptionListDescription>
                                        </DescriptionListGroup>

                                        <DescriptionListGroup>
                                            <DescriptionListTerm>Backups</DescriptionListTerm>
                                            <DescriptionListDescription>11</DescriptionListDescription>
                                        </DescriptionListGroup>

                                        <DescriptionListGroup>
                                            <DescriptionListTerm>Destinations</DescriptionListTerm>
                                            <DescriptionListDescription>2</DescriptionListDescription>
                                        </DescriptionListGroup>
                                    </DescriptionList>
                                </CardBody>
                            </Card>
                        </GridItem>

                        <GridItem span={9}>
                            <Card>
                                <CardTitle>Status</CardTitle>

                                <CardBody>
                                    <Gallery>

                                        <Flex>
                                            <FlexItem spacer={{ default: 'spacerSm' }}>
                                                <Icon status="success">
                                                    <CheckCircleIcon />
                                                </Icon>
                                            </FlexItem>
                                            <FlexItem>
                                                <span>8 successful</span>
                                            </FlexItem>
                                        </Flex>

                                        <Flex>
                                            <FlexItem spacer={{ default: 'spacerSm' }}>
                                                <Icon status="warning">
                                                    <DotCircleIcon />
                                                </Icon>
                                            </FlexItem>
                                            <FlexItem>
                                                <span>1 in progress</span>
                                            </FlexItem>
                                        </Flex>

                                        <Flex>
                                            <FlexItem spacer={{ default: 'spacerSm' }}>
                                                <Icon status="danger">
                                                    <ExclamationCircleIcon />
                                                </Icon>
                                            </FlexItem>
                                            <FlexItem>
                                                <span>2 failed</span>
                                            </FlexItem>
                                        </Flex>

                                    </Gallery>
                                </CardBody>
                            </Card>
                        </GridItem>
                    </Grid>
                </PageSection>
            </AppLayout>
        </>
    );
};

export default Home;
