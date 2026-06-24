import os
import platform

from conan import ConanFile
from conan.tools.cmake import CMake, CMakeToolchain, cmake_layout, CMakeDeps
from conan.tools.build import check_min_cppstd
from conan.tools.files import rmdir


class QuasarProtobufsRecipe(ConanFile):
    name = "quasar.protobufs"
    version = "0.5.0"
    description = "Protobufs pack related to QuaSAR"
    author = "whs31 <whs31@github.io>"
    topics = ("grpc", "protocol", "network")
    settings = "os", "arch", "compiler", "build_type"
    options = {"grpc": [True, False]}
    default_options = {"grpc": True}
    exports = "CMakeLists.txt", "cmake/*"
    exports_sources = "*", "!build/*", "!CMakeUserPresets.json", "!.git/*", "!target/*"

    user = "quasar"
    channel = "dev"

    @property
    def _min_cppstd(self):
        return "20"

    def requirements(self):
        if self.options.grpc:
            self.requires(
                "grpc/1.72.0", transitive_libs=True, transitive_headers=True, force=True
            )
        self.requires("protobuf/5.27.0", transitive_libs=True, transitive_headers=True)

    def build_requirements(self):
        self.tool_requires("cmake/[>=3.23.5]")
        self.tool_requires("protobuf/<host_version>")

    def layout(self):
        cmake_layout(self)

    def validate(self):
        if self.settings.get_safe("compiler.cppstd"):
            check_min_cppstd(self, self._min_cppstd)

    def generate(self):
        deps = CMakeDeps(self)
        deps.generate()
        tc = CMakeToolchain(self)
        tc.variables["BUILD_SHARED_LIBS"] = False
        tc.variables["QUASAR_PROTOBUFS_GRPC"] = self.options.grpc
        tc.generate()

    def build(self):
        cmake = CMake(self)
        cmake.configure()
        cmake.build()

    def package(self):
        cmake = CMake(self)
        cmake.install()
        rmdir(self, os.path.join(self.package_folder, "lib", "cmake"))
        rmdir(self, os.path.join(self.package_folder, "lib", "pkgconfig"))
        rmdir(self, os.path.join(self.package_folder, "res"))

    def package_info(self):
        self.cpp_info.set_property("cmake_file_name", "quasar.protobufs")
        self.cpp_info.set_property("cmake_target_name", "quasar::protobufs")
        self.cpp_info.libs = ["quasar-protobufs"]
        self.cpp_info.requires = ["protobuf::protobuf"]
        if self.options.grpc:
            self.cpp_info.requires.append("grpc::grpc")
        self.cpp_info.resdirs = ["share"]