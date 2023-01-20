#include <usdffi/ffi.h>

#include <composition.hh>
#include <filesystem>
#include <iostream>
#include <stream-reader.hh>
#include <tinyusdz.hh>
#include <usda-reader.hh>

int main()
{
    // Blender 3.1 で Cube を usda 形式で出力した結果をコピー
    const auto* pData = R"(#usda 1.0

def Xform "Cube"
{
    def Mesh "Cube"
    {
        uniform bool doubleSided = 1
        int[] faceVertexCounts = [4, 4, 4, 4, 4, 4]
        int[] faceVertexIndices = [0, 4, 6, 2, 3, 2, 6, 7, 7, 6, 4, 5, 5, 1, 3, 7, 1, 0, 2, 3, 5, 4, 0, 1]
        rel material:binding = </_materials/Material>
        normal3f[] normals = [(0, 0, 1), (0, 0, 1), (0, 0, 1), (0, 0, 1), (0, -1, 0), (0, -1, 0), (0, -1, 0), (0, -1, 0), (-1, 0, 0), (-1, 0, 0), (-1, 0, 0), (-1, 0, 0), (0, 0, -1), (0, 0, -1), (0, 0, -1), (0, 0, -1), (1, 0, 0), (1, 0, 0), (1, 0, 0), (1, 0, 0), (0, 1, 0), (0, 1, 0), (0, 1, 0), (0, 1, 0)] (
            interpolation = "faceVarying"
        )
        point3f[] points = [(1, 1, 1), (1, 1, -1), (1, -1, 1), (1, -1, -1), (-1, 1, 1), (-1, 1, -1), (-1, -1, 1), (-1, -1, -1)]
        texCoord2f[] primvars:UVMap = [(0.625, 0.5), (0.875, 0.5), (0.875, 0.75), (0.625, 0.75), (0.375, 0.75), (0.625, 0.75), (0.625, 1), (0.375, 1), (0.375, 0), (0.625, 0), (0.625, 0.25), (0.375, 0.25), (0.125, 0.5), (0.375, 0.5), (0.375, 0.75), (0.125, 0.75), (0.375, 0.5), (0.625, 0.5), (0.625, 0.75), (0.375, 0.75), (0.375, 0.25), (0.625, 0.25), (0.625, 0.5), (0.375, 0.5)] (
            interpolation = "faceVarying"
        )
        uniform token subdivisionScheme = "none"
    }
})";

    auto* pStreamReader =
        StreamReader_New(reinterpret_cast<const uint8_t*>(pData), std::strlen(pData) + 1, false);

    {
        auto* pUsdaReader = USDAReader_New(pStreamReader);
        USDAReader_Read(pUsdaReader, tinyusdz::LoadState::Toplevel);
        USDAReader_ReconstructStage(pUsdaReader);

        const tinyusdz::Stage* pStage = nullptr;
        USDAReader_GetStage(pUsdaReader, &pStage);
        std::cout << pStage->ExportToString() << std::endl;

        tinyusdz::Path path{"/Cube", ""};
        const tinyusdz::Prim* pPrim{nullptr};
        const auto isSuccess = Stage_FindPrimitiveAtPath(pStage, &pPrim, &path);

        // XForm
        const auto* pXform = Prim_AsXForm(pPrim);
//        pXform->GetLocalMatrix(); // 行列が取れる

        for (const auto& child : pPrim->children()) {
            const auto* pGeomMesh = Prim_AsGeomMesh(&child);
            if (pGeomMesh == nullptr) {
                continue;;
            }

            std::cout << "Name      : " << pGeomMesh->get_name() << std::endl;
            std::cout << "VertexCount: " << pGeomMesh->get_points().size() << std::endl;
            std::cout << "IndexCount: " << pGeomMesh->get_faceVertexIndices().size() << std::endl;
        }

        USDAReader_Delete(pUsdaReader);
    }

    StreamReader_Delete(pStreamReader);

    return EXIT_SUCCESS;
}
