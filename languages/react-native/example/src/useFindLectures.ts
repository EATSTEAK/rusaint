import {
  CourseScheduleApplicationBuilder,
  Lecture,
  LectureCategory,
  LectureCategoryBuilder,
  SemesterType,
  USaintSessionBuilder,
  type CourseScheduleApplicationInterface,
} from '@rusaint/react-native';
import { useRef, useState, useEffect } from 'react';

const session = new USaintSessionBuilder().anonymous();

export const useFindLectures = (
  year: number,
  semester: SemesterType,
  category: LectureCategory
) => {
  const clientRef = useRef<CourseScheduleApplicationInterface | null>(null);
  const [result, setResult] = useState<Lecture[]>([]);
  useEffect(() => {
    (async () => {
      const client = await new CourseScheduleApplicationBuilder().build(
        session
      );
      clientRef.current = client;
    })();
  }, []);

  useEffect(() => {
    (async () => {
      let result = await clientRef.current?.findLectures(
        year,
        semester,
        category
      );
      setResult(result || []);
      console.log('Lectures fetched:', result);
    })();
  }, [year, semester, category]);
  return result;
};
