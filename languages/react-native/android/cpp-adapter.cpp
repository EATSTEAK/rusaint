#include <jni.h>
#include "rusaint-react-native.h"

extern "C"
JNIEXPORT jdouble JNICALL
Java_com_rusaint_reactnative_ReactNativeModule_nativeMultiply(JNIEnv *env, jclass type, jdouble a, jdouble b) {
    return rusaint_reactnative::multiply(a, b);
}
