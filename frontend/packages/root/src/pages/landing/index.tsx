import { FC } from "react";

import CarasolSection from "./carasol-section";
import GettingStart from "./getting-start";
import FeatureSection from "./feature-section";
import ProductSection from "./product-section";
import WhatWeProvide from "./provide-section";
import TestimonialSection from "./testimonial-section";

const LandingPage: FC = () => {
    return (
        <>
        <CarasolSection />
        <GettingStart />
        <ProductSection />
        <FeatureSection />
        <WhatWeProvide />
        <TestimonialSection />
        </>
    )
};

export default LandingPage;