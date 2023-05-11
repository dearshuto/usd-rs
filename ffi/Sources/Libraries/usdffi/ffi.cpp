#include <io-util.hh>
#include <stream-reader.hh>
#include <tinyusdz.hh>
#include <tydra/scene-access.hh>
#include <usda-reader.hh>
#include <vector>

// StreamReader
extern "C" tinyusdz::StreamReader *StreamReader_New(const uint8_t *binary,
                                                    uint64_t size,
                                                    bool isSwapEndian)
{
    return new tinyusdz::StreamReader{binary, size, isSwapEndian};
}

extern "C" void StreamReader_Delete(tinyusdz::StreamReader *pReader) { delete pReader; }
//-----------------------------------------------------------------------------

// USDAReader の API
extern "C" tinyusdz::usda::USDAReader *USDAReader_New(tinyusdz::StreamReader *pStreamReader)
{
    auto *pInstance = new tinyusdz::usda::USDAReader{pStreamReader};
    return pInstance;
}

extern "C" void USDAReader_Delete(tinyusdz::usda::USDAReader *pReader) { delete pReader; }

extern "C" bool USDAReader_Read(tinyusdz::usda::USDAReader *pReader, tinyusdz::LoadState state)
{
    const auto isSuccess = pReader->Read(state);
    return isSuccess;
}

extern "C" bool USDAReader_ReconstructStage(tinyusdz::usda::USDAReader *pReader)
{
    const auto isSuccess = pReader->ReconstructStage();
    return isSuccess;
}

extern "C" void USDAReader_GetStage(const tinyusdz::usda::USDAReader *pReader,
                                    const tinyusdz::Stage **pOutStage)
{
    *pOutStage = &pReader->GetStage();
}
//-----------------------------------------------------------------------------

// Stage の API
extern "C" tinyusdz::Stage *Stage_New()
{
    auto *pInstance = new tinyusdz::Stage;
    return pInstance;
}

extern "C" void Stage_Delete(tinyusdz::Stage *pStage) { delete pStage; }

extern "C" bool Stage_FindPrimitiveAtPath(const tinyusdz::Stage *pStage,
                                          const tinyusdz::Prim **ppOutPrimitive,
                                          const tinyusdz::Path *pPath)
{
    std::string err;
    const auto isSuccess = pStage->find_prim_at_path(*pPath, *ppOutPrimitive, &err);
    return isSuccess;
}
//-----------------------------------------------------------------------------

// Path の API
extern "C" tinyusdz::Path *Path_New(const char *pAbsolutePrimitivePath, const char *pPropetyPath)
{
    // tinyusdz::Path path(/* absolute prim path */ "/root",
    //                     /* property path */ "");
    auto *pInstance = new tinyusdz::Path{pAbsolutePrimitivePath, pPropetyPath};
    return pInstance;
}

extern "C" void Path_Delete(tinyusdz::Path *pInstance) { delete pInstance; }
//-----------------------------------------------------------------------------

// Prim の API
extern "C" const tinyusdz::Xform *Prim_AsXForm(const tinyusdz::Prim *pPrim)
{
    auto *pXForm = pPrim->as<tinyusdz::Xform>();
    return pXForm;
}

extern "C" const tinyusdz::GeomMesh *Prim_AsGeomMesh(const tinyusdz::Prim *pPrim)
{
    auto *pGeomMesh = pPrim->as<tinyusdz::GeomMesh>();
    return pGeomMesh;
}

extern "C" const tinyusdz::GeomCube *Prim_AsGeomCube(const tinyusdz::Prim *pPrim)
{
    auto *pGeomCube = pPrim->as<tinyusdz::GeomCube>();
    return pGeomCube;
}

extern "C" int64_t Prim_PrimId(const tinyusdz::Prim *pPrim)
{
    const auto id = pPrim->prim_id();
    return id;
}

extern "C" void Prim_GetAbsolutePath(const tinyusdz::Prim *pPrim, tinyusdz::Path *pOutPath)
{
    const auto &absolutePath = pPrim->absolute_path();
    *pOutPath                = absolutePath;
}

extern "C" int Prim_GetChildCount(const tinyusdz::Prim *pPrim)
{
    const auto &children = pPrim->children();
    return static_cast<int>(children.size());
}

extern "C" const tinyusdz::Prim *Prim_GetChild(const tinyusdz::Prim *pPrim, int index)
{
    auto &children = pPrim->children();
    auto *pChild   = &children[index];
    return pChild;
}

extern "C" bool Prim_GetAttribute(tinyusdz::Attribute *pOutInstance,
                                  const tinyusdz::Prim *pPrimitive,
                                  const char *attributeName /*ex: "points" */)
{
    std::string error;
    const auto isSuccess =
        tinyusdz::tydra::GetAttribute(*pPrimitive, attributeName, pOutInstance, &error);
    return isSuccess;
}

//-----------------------------------------------------------------------------

// GeomPrimvar の API
extern "C" tinyusdz::GeomPrimvar *GeomPrimvar_New() { return new tinyusdz::GeomPrimvar; }

extern "C" void GeomPrimvar_Delete(tinyusdz::GeomPrimvar *pInstance) { delete pInstance; }

extern "C" bool GeomPrimvar_HasIndices(const tinyusdz::GeomPrimvar *pInstance)
{
    const auto hasIndices = pInstance->has_indices();
    return hasIndices;
}

extern "C" bool GeomPrimvar_HasElementSize(const tinyusdz::GeomPrimvar *pInstance)
{
    const auto hasElementSize = pInstance->has_elementSize();
    return hasElementSize;
}
//-----------------------------------------------------------------------------

// GeomMesh の API
extern "C" bool GeomMesh_HasPrimvar(const tinyusdz::GeomMesh *pGeomMesh, const char *pName)
{
    const auto hasPrimVar = pGeomMesh->has_primvar(pName);
    return hasPrimVar;
}

extern "C" int GeomMesh_GetPointCount(const tinyusdz::GeomMesh *pGeomMesh)
{
    const auto &points = pGeomMesh->get_points();
    return static_cast<int>(points.size());
}

extern "C" void GeomMesh_GetPoint(
    const tinyusdz::GeomMesh *pGeomMesh, float *pOutX, float *pOutY, float *pOutZ, int index)
{
    const auto &points = pGeomMesh->get_points();
    const auto &point  = points[index];
    *pOutX             = point.x;
    *pOutY             = point.y;
    *pOutZ             = point.z;
}

extern "C" void GeomMesh_GetPoints(const tinyusdz::GeomMesh *pGeomMesh,
                                   const tinyusdz::value::point3f **ppOutHead,
                                   int *pOutCount)
{
    const auto &points = pGeomMesh->get_points();
    *ppOutHead         = points.data();
    *pOutCount         = static_cast<int>(points.size());
}

extern "C" int GeomMesh_GetIndexCount(const tinyusdz::GeomMesh *pGeomMesh)
{
    const auto count = pGeomMesh->get_faceVertexIndices().size();
    return count;
}

extern "C" void GeomMesh_GetIndex(
    const tinyusdz::GeomMesh *pGeomMesh, int32_t* pOutIndex, int index)
{
    const auto indices = pGeomMesh->get_faceVertexIndices();
    *pOutIndex = indices[index];
}

extern "C" int GeomMesh_GetNormalCount(const tinyusdz::GeomMesh *pGeomMesh)
{
    const auto &normals_opt = pGeomMesh->normals;
    if (normals_opt.is_value_empty()) {
        return 0;
    }

    return pGeomMesh->get_normals().size();
}

extern "C" void GeomMesh_GetNormal(
    const tinyusdz::GeomMesh *pGeomMesh, float *pOutX, float *pOutY, float *pOutZ, int index)
{
    const auto &normals_opt = pGeomMesh->normals;
    if (normals_opt.is_value_empty()) {
        return;
    }

    const auto normal = pGeomMesh->get_normals()[index];
    *pOutX            = normal.x;
    *pOutY            = normal.y;
    *pOutZ            = normal.z;
}

extern "C" void GeomMesh_GetFaceVertexCounts(const tinyusdz::GeomMesh *pGeomMesh,
                                             const int **ppOutHead,
                                             int *pOutCount)
{
    const auto &vertexCounts = pGeomMesh->get_faceVertexCounts();
    *ppOutHead               = vertexCounts.data();
    *pOutCount               = static_cast<int>(vertexCounts.size());
}

extern "C" void GeomMesh_GetFaceVertexIndices(const tinyusdz::GeomMesh *pGeomMesh,
                                              const int **ppOutHead,
                                              int *pOutCount)
{
    const auto &vertexIndices = pGeomMesh->get_faceVertexIndices();
    *ppOutHead                = vertexIndices.data();
    *pOutCount                = static_cast<int>(vertexIndices.size());
}
//-----------------------------------------------------------------------------

// Attribute の API
extern "C" tinyusdz::Attribute *Attribute_New()
{
    auto *pInstance = new tinyusdz::Attribute;
    return pInstance;
}

extern "C" void Attribute_Delete(tinyusdz::Attribute *pInstance) { delete pInstance; }

extern "C" bool Attribute_IsTimesamples(const tinyusdz::Attribute *pInstance)
{
    const auto isTimesamples = pInstance->is_timesamples();
    return isTimesamples;
}

extern "C" bool Attribute_IsValue(const tinyusdz::Attribute *pInstance)
{
    const auto isValue = pInstance->is_value();
    return isValue;
}

extern "C" bool Attribute_GetValuePpoint3f(const tinyusdz::Attribute *pInstance)
{
    std::vector<tinyusdz::value::point3f> points;
    const auto isSuccess = pInstance->get_value(&points);
    return isSuccess;
}
//-----------------------------------------------------------------------------

// Xform の API
extern "C" void Xform_GetLocalMatrix(const tinyusdz::Xform *pInstance,
                                     float *pArray,
                                     [[maybe_unused]] int count)
{
    const auto &matrix = pInstance->GetLocalMatrix();
    auto *pData        = &matrix->m[0][0];
    for (auto index = 0; index < 16; ++index) {
        pArray[index] = pData[index];
    }
}

extern "C" const tinyusdz::XformOp *Xform_GetOp(const tinyusdz::Xform *pInstance, int index)
{
    auto *pOp = &pInstance->xformOps[index];
    return pOp;
}

extern "C" int32_t Xform_GetOpCount(const tinyusdz::Xform *pInstance)
{
    auto count = static_cast<int>(pInstance->xformOps.size());
    return count;
}

// XformOp の API
extern "C" int32_t XformOp_GetType(const tinyusdz::XformOp *pInstance)
{
    return static_cast<int32_t>(pInstance->op_type);
}

extern "C" bool XformOp_IsTimeSamples(const tinyusdz::XformOp *pInstance)
{
    return pInstance->is_timesamples();
}

extern "C" const tinyusdz::value::TimeSamples *XformOp_GetTimeSamples(
    const tinyusdz::XformOp *pInstance)
{
    return &pInstance->get_var().ts_raw();
}

extern "C" bool XformOp_TimeSamplesEmpty(const tinyusdz::XformOp *pInstance)
{
    auto &&timeSamples = pInstance->get_timesamples().value();
    return timeSamples.empty();
}

extern "C" int32_t XformOp_TimeSamplesSize(const tinyusdz::XformOp *pInstance)
{
    auto &&timeSamples = pInstance->get_timesamples().value();
    return timeSamples.size();
}
//-----------------------------------------------------------------------------

// TimeSamples の API
extern "C" bool TimeSamples_Empty(const tinyusdz::value::TimeSamples *pInstance)
{
    return pInstance->empty();
}

extern "C" int32_t TimeSamples_Size(const tinyusdz::value::TimeSamples *pInstance)
{
    return pInstance->size();
}

extern "C" double TimeSamples_GetTime(const tinyusdz::value::TimeSamples *pInstance, int index)
{
    const auto time = pInstance->get_time(index).value();
    return time;
}

extern "C" void TimeSamples_GetValue(tinyusdz::value::Value *pOutValue,
                                     const tinyusdz::value::TimeSamples *pInstance,
                                     int index)
{
    const auto v = pInstance->get_value(index).value();
    std::memcpy(pOutValue, &v, sizeof(tinyusdz::value::Value));
}
//-----------------------------------------------------------------------------

// Value の API
extern "C" tinyusdz::value::Value *Value_New() { return new tinyusdz::value::Value; }

extern "C" void Value_Delete(tinyusdz::value::Value *pInstance) { delete pInstance; }

extern "C" float Value_AsFloat(const tinyusdz::value::Value *pInstance)
{
    return *pInstance->as<float>();
}
//-----------------------------------------------------------------------------

// GeomCube の API
extern "C" void a(tinyusdz::GeomCube* pGeomCube) {
    
}
//-----------------------------------------------------------------------------
