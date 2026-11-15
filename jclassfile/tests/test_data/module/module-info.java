module sample {
    requires transitive java.logging;
    requires static java.compiler;

    exports sample.api;
    exports sample.impl to java.logging, java.compiler;

    opens sample.api;
    opens sample.impl to java.logging;

    uses sample.api.Service;
    provides sample.api.Service with sample.impl.FirstService, sample.impl.SecondService;
}
