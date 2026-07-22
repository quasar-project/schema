import os

from conan import ConanFile
from conan.tools.build import check_min_cppstd
from conan.tools.cmake import CMake, CMakeDeps, CMakeToolchain, cmake_layout
from conan.tools.env import VirtualRunEnv
from conan.tools.files import rmdir


class QuaSARSchemaRecipe(ConanFile):
    name = "quasar_schema"
    version = "1.6.0"
    package_type = "shared-library"
    description = "QuaSAR Schema protobuf contract"
    author = "whs31 <whs31@github.io>"
    topics = ("protobuf", "protocol", "network", "zeromq")
    settings = "os", "arch", "compiler", "build_type"
    exports = "CMakeLists.txt"
    exports_sources = (
        "*",
        "!.conan2/*",
        "!build/*",
        "!CMakeUserPresets.json",
        "!.git/*",
        "!.idea/*",
        "!target/*",
        "!test_package/build/*",
        "!test_package/CMakeUserPresets.json",
    )

    user = "quasar"
    channel = "dev"

    python_requires = "conan_helpers/0.2@radar/dev"
    python_requires_extend = "conan_helpers.Base"

    @property
    def _min_cppstd(self):
        return "20"

    def requirements(self):
        self.req(
            "mms_ipc_core@radar/dev",
            transitive_libs=True,
            transitive_headers=True,
        )
        self.req(
            "protobuf",
            options={"shared": True},
            transitive_libs=True,
            transitive_headers=True,
        )
        self.req(
            "abseil",
            options={"shared": True},
            transitive_libs=True,
            transitive_headers=True,
        )

    def build_requirements(self):
        self.req("cmake", tool=True)
        self.req("protobuf", tool=True)

    def layout(self):
        cmake_layout(self)

    def validate(self):
        if self.settings.get_safe("compiler.cppstd"):
            check_min_cppstd(self, self._min_cppstd)

    def configure(self):
        self.options["abseil"].shared = True
        self.options["protobuf"].shared = True

    def generate(self):
        deps = CMakeDeps(self)
        deps.generate()

        tc = CMakeToolchain(self)
        tc.shared = True

        mms_dep = self.dependencies["mms_ipc_core"]
        tc.variables["QUASAR_SCHEMA_MMS_PROTO_DIR"] = os.path.join(
            mms_dep.package_folder,
            "schema",
        )
        tc.generate()

        ms = VirtualRunEnv(self)
        ms.generate()

    def build(self):
        cmake = CMake(self)
        cmake.configure()
        cmake.build()
        if not self.conf.get(
            "tools.build:skip_test", default=False, check_type=bool
        ):
            cmake.test()

    def package(self):
        cmake = CMake(self)
        cmake.install()
        rmdir(self, os.path.join(self.package_folder, "lib", "cmake"))
        rmdir(self, os.path.join(self.package_folder, "lib", "pkgconfig"))

    def package_info(self):
        self.cpp_info.set_property("cmake_file_name", "QuaSARSchema")

        ffi = self.cpp_info.components["ffi"]
        ffi.set_property("cmake_target_name", "quasar::schema_ffi")
        ffi.includedirs = ["include"]
        ffi.libdirs = []
        ffi.bindirs = []

        schema = self.cpp_info.components["schema"]
        schema.set_property("cmake_target_name", "quasar::schema")
        schema.libs = ["quasar_schema"]
        schema.requires = [
            "ffi",
            "mms_ipc_core::mms_ipc_core",
            "protobuf::protobuf",
            "abseil::abseil",
        ]
        schema.resdirs = ["schema"]
