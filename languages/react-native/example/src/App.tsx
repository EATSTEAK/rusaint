import {
  CourseScheduleApplicationBuilder,
  type CourseScheduleApplicationInterface,
  Lecture,
  LectureCategoryBuilder,
  SemesterType,
  USaintSessionBuilder,
} from '@rusaint/react-native';
import { Text, View, StyleSheet } from 'react-native';
import { useEffect, useRef, useState } from 'react';
import { useFindLectures } from './useFindLectures';

const category = new LectureCategoryBuilder().major(
  'IT대학',
  '글로벌미디어학부',
  undefined
);

export default function App() {
  const result = useFindLectures(2025, SemesterType.One, category);
  return (
    <View style={styles.container}>
      <Text>Result: {JSON.stringify(result[0])}</Text>
    </View>
  );
}

const styles = StyleSheet.create({
  container: {
    flex: 1,
    alignItems: 'center',
    justifyContent: 'center',
  },
});
