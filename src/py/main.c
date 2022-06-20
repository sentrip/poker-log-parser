#define PY_SSIZE_T_CLEAN
#include <Python.h>

#include <stdio.h>

extern const char* pklp_str_to_json(const char* data);
extern const char* pklp_path_to_json(const char* path);
extern void pklp_path_to_json_file(const char* path, const char* output_path);

extern const char* pklp_strs_to_json(const char* const* data, size_t n);
extern const char* pklp_paths_to_json(const char* const* paths, size_t n);
extern void pklp_paths_to_json_file(const char* const* paths, size_t n, const char* output_path);


static PyObject* str_to_json(PyObject *self, PyObject *args)
{
    const char *s;
    if (!PyArg_ParseTuple(args, "s", &s))
        return NULL;
    const char* j = pklp_str_to_json(s);
    PyObject* o = Py_BuildValue("s", j);
    free((void*)j);
    return o;
}

static PyObject* path_to_json(PyObject *self, PyObject *args)
{
    const char *s;
    if (!PyArg_ParseTuple(args, "s", &s))
        return NULL;
    const char* j = pklp_path_to_json(s);
    PyObject* o = Py_BuildValue("s", j);
    free((void*)j);
    return o;
}

static PyObject* path_to_json_file(PyObject *self, PyObject *args)
{
    const char *s, *out;
    if (!PyArg_ParseTuple(args, "ss", &s, &out))
        return NULL;
    pklp_path_to_json_file(s, out);
    return NULL;
}

static const char** get_strings_from_python(PyObject *args, int* len) 
{
    PyObject * list_obj;

    if (!PyArg_ParseTuple( args, "O!", &PyList_Type, &list_obj )) 
        return NULL;

    int list_len = PyList_Size(list_obj);
    if (list_len < 0)
        return NULL;

    *len = list_len;
    const char** strings = (const char**)malloc(list_len * sizeof(const char*));
    for (int i = 0; i < list_len; ++i) {
        PyObject * str_obj = PyList_GetItem(list_obj, i);
        strings[i] = PyUnicode_AsUTF8(str_obj);
    }

    return strings;
}

static PyObject* strs_to_json(PyObject *self, PyObject *args)
{
    int list_len;
    const char** strings = get_strings_from_python(args, &list_len);
    const char* j = pklp_strs_to_json(strings, (size_t)list_len);
    PyObject* o = Py_BuildValue("s", j);
    free((void*)j);
    free(strings);
    return o;
}

static PyObject* paths_to_json(PyObject *self, PyObject *args)
{
    int list_len;
    const char** paths = get_strings_from_python(args, &list_len);
    const char* j = pklp_paths_to_json(paths, (size_t)list_len);
    PyObject* o = Py_BuildValue("s", j);
    free((void*)j);
    free(paths);
    return o;
}

static PyObject* paths_to_json_file(PyObject *self, PyObject *args)
{
    const char* out;
    PyObject * list_obj;

    if (!PyArg_ParseTuple( args, "O!s", &PyList_Type, &list_obj, &out)) 
        return NULL;

    int list_len = PyList_Size(list_obj);
    if (list_len < 0)
        return NULL;

    const char** paths = (const char**)malloc(list_len * sizeof(const char*));
    for (int i = 0; i < list_len; ++i) {
        PyObject * str_obj = PyList_GetItem(list_obj, i);
        paths[i] = PyUnicode_AsUTF8(str_obj);
    }

    pklp_paths_to_json_file(paths, (size_t)list_len, out);
    free(paths);
    return NULL;
}

static PyMethodDef PklpMethods[] = {
    {"str_to_json",         str_to_json,         METH_VARARGS, "Convert string to json"},
    {"path_to_json",        path_to_json,        METH_VARARGS, "Convert file path to json"},
    {"path_to_json_file",   path_to_json_file,   METH_VARARGS, "Convert file path to json file"},
    {"strs_to_json",        strs_to_json,        METH_VARARGS, "Convert list of strings to json"},
    {"paths_to_json",       paths_to_json,       METH_VARARGS, "Convert list of file paths to json"},
    {"paths_to_json_file",  paths_to_json_file,  METH_VARARGS, "Convert list of file paths to json file"},
    {NULL, NULL, 0, NULL}
};

static struct PyModuleDef pklpmodule = {
    PyModuleDef_HEAD_INIT,
    "pklp",
    "A blazingly-fast library for converting poker hand history text path into JSON",
    -1,
    PklpMethods
};

PyMODINIT_FUNC PyInit_pklp(void)
{
    return PyModule_Create(&pklpmodule);
}

int main() {
    return 0;
}

