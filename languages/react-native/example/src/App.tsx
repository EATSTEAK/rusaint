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

const session = new USaintSessionBuilder().anonymous();

export default function App() {
  console.log('App started');
  const clientRef = useRef<CourseScheduleApplicationInterface | null>(null);
  const [result, setResult] = useState<Lecture[]>([]);
  useEffect(() => {
    (async () => {
      console.log('Initializing client');
      const client = await new CourseScheduleApplicationBuilder().build(
        session
      );
      clientRef.current = client;
      console.log('Client initialized');
      const category = new LectureCategoryBuilder().major(
        'IT대학',
        '글로벌미디어학부',
        undefined
      );
      let result = await clientRef.current?.findLectures(
        2025,
        SemesterType.One,
        category
      );
      setResult(result || []);
      console.log('Lectures fetched:', result);
    })();
  }, []);
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
