import os

from conan import ConanFile
from conan.tools.build import check_min_cppstd
from conan.tools.cmake import CMake, CMakeDeps, CMakeToolchain, cmake_layout
from conan.tools.env import VirtualRunEnv
from conan.tools.files import rmdir


class QuaSARSchemaRecipe(ConanFile):
    name = "quasar_schema"
    version = "1.1.0"
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
            "mms@radar/dev",
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

        mms_dep = self.dependencies["mms"]
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

    def package(self):
        cmake = CMake(self)
        cmake.install()
        rmdir(self, os.path.join(self.package_folder, "lib", "cmake"))
        rmdir(self, os.path.join(self.package_folder, "lib", "pkgconfig"))

    def package_info(self):
        self.cpp_info.set_property("cmake_file_name", "QuaSARSchema")
        self.cpp_info.set_property(
            "cmake_target_name", "quasar::schema"
        )
        self.cpp_info.libs = ["quasar_schema"]
        self.cpp_info.requires = [
            "mms::Protocol",
            "protobuf::protobuf",
            "abseil::abseil",
        ]
        self.cpp_info.resdirs = ["schema"]