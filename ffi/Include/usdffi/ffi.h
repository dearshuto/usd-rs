#include <cstdint>

namespace tinyusdz {
class StreamReader;
class GeomMesh;
class Stage;
class Path;
class Prim;
class Xform;
enum class LoadState;
}  // namespace tinyusdz

namespace tinyusdz::usda {
class USDAReader;
}

extern "C" tinyusdz::StreamReader *StreamReader_New(const uint8_t *binary,
                                                    uint64_t size,
                                                    bool isSwapEndian);

extern "C" void StreamReader_Delete(tinyusdz::StreamReader *pReader);

extern "C" tinyusdz::usda::USDAReader *USDAReader_New(tinyusdz::StreamReader *pStreamReader);

extern "C" void USDAReader_Delete(tinyusdz::usda::USDAReader *pReader);

extern "C" bool USDAReader_Read(tinyusdz::usda::USDAReader *pReader, tinyusdz::LoadState state);

extern "C" bool USDAReader_ReconstructStage(tinyusdz::usda::USDAReader *pReader);

extern "C" void USDAReader_GetStage(const tinyusdz::usda::USDAReader *pReader,
                                    const tinyusdz::Stage **ppOutStage);

extern "C" bool Stage_FindPrimitiveAtPath(const tinyusdz::Stage *pStage,
                                          const tinyusdz::Prim **ppOutPrimitive,
                                          const tinyusdz::Path *pPath);

extern "C" const tinyusdz::GeomMesh *Prim_AsGeomMesh(const tinyusdz::Prim *pPrim);

extern "C" const tinyusdz::Xform *Prim_AsXForm(const tinyusdz::Prim *pPrim);

extern "C" int64_t Prim_PrimId(const tinyusdz::Prim *pPrim);

extern "C" bool GeomMesh_HasPrimvar(const tinyusdz::GeomMesh *pGeomMesh, const char *pName);

extern "C" int GeomMesh_GetPointCount(const tinyusdz::GeomMesh *pGeomMesh);

extern "C" void GeomMesh_GetPoint(
    const tinyusdz::GeomMesh *pGeomMesh, float *pOutX, float *pOutY, float *pOutZ, int index);
