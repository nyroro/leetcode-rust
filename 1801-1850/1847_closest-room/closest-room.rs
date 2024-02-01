
// 定义一个结构体来表示房间的信息

#[derive(Debug, Eq, PartialEq)]
struct Room {
    id: i32,
    size: i32,
}

impl Solution {
    pub fn closest_room(rooms: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        // 将房间信息转换为Room结构体的向量

        let mut room_list: Vec<Room> = rooms

            .iter()
            .map(|room| Room {
                id: room[0],
                size: room[1],
            })
            .collect();

        // 对房间按照大小进行排序

        room_list.sort_by(|a, b| a.size.cmp(&b.size));

        // 定义一个函数来查找满足条件的房间

        let find_closest_room = |preferred: i32, min_size: i32| -> i32 {
            let mut left = 0;
            let mut right = room_list.len() as i32 - 1;
            let mut result = -1;

            while left <= right {
                let mid = left + (right - left) / 2;
                if room_list[mid as usize].size < min_size {
                    left = mid + 1;
                } else {
                    if result == -1

                        || (i32::abs(room_list[mid as usize].id - preferred)
                            < i32::abs(room_list[result as usize].id - preferred))
                        || ((i32::abs(room_list[mid as usize].id - preferred)
                            == i32::abs(room_list[result as usize].id - preferred))
                            && (room_list[mid as usize].id < room_list[result as usize].id))
                    {
                        result = mid;
                    }
                    right = mid - 1;
                }
            }

            if result == -1 {
                -1

            } else {
                room_list[result as usize].id

            }
        };

        // 对每个查询进行处理

        let mut answer: Vec<i32> = Vec::new();
        for query in queries {
            let preferred = query[0];
            let min_size = query[1];
            let closest_room = find_closest_room(preferred, min_size);
            answer.push(closest_room);
        }

        answer

    }
}
