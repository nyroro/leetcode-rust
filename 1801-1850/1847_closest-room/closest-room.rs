
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

        room_list.sort_by(|a, b| b.size.cmp(&a.size).then_with(|| a.id.cmp(&b.id)));

        // 定义一个函数来查找满足条件的房间

        let find_closest_room = |preferred: i32, min_size: i32| -> i32 {
            let mut result = -1;
            let mut min_diff = i32::max_value();

            for room in &room_list {
                if room.size >= min_size {
                    let diff = (room.id - preferred).abs();
                    if diff < min_diff || (diff == min_diff && room.id < result) {
                        min_diff = diff;
                        result = room.id;
                    }
                }
            }

            result

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
