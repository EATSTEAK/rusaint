import {
  CourseScheduleApplicationBuilder,
  Lecture,
  LectureCategory,
  LectureCategoryBuilder,
  SemesterType,
  USaintSessionBuilder,
  type CourseScheduleApplicationLike,
} from '@rusaint/react-native';
import { useState, useEffect } from 'react';

const session = new USaintSessionBuilder().anonymous();

export const useFindLectures = (
  year: number,
  semester: SemesterType,
  category: LectureCategory
) => {
  const [result, setResult] = useState<Lecture[]>([]);

  useEffect(() => {
    let cancelled = false;

    (async () => {
      const client = await new CourseScheduleApplicationBuilder().build(
        session
      );
      const result = await client.findLectures(year, semester, category);

      if (cancelled) {
        return;
      }

      setResult(result);
      console.log('Lectures fetched:', result);
    })();

    return () => {
      cancelled = true;
    };
  }, [year, semester, category]);

  return result;
};
