fn main() {
    cc::Build::new()
        .cpp(true)
        .file("job.cpp")
        .flag("-flto=thin")
        .compile("libjob.a");
}