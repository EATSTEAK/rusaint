import {
  CourseScheduleApplicationBuilder,
  LectureCategoryBuilder,
  SemesterType,
  USaintSessionBuilder,
} from '@rusaint/react-native';
import { Text, View, StyleSheet } from 'react-native';
import { use } from 'react';

const session = new USaintSessionBuilder().anonymous();

const client = await new CourseScheduleApplicationBuilder().build(session);

export default function App() {
  const category = new LectureCategoryBuilder().major(
    'IT대학',
    '글로벌미디어학부',
    undefined
  );
  let result = use(client.findLectures(2025, SemesterType.One, category));
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
