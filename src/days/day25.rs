use crate::prelude::*;
use std::str::FromStr;

pub fn run(data: &AocData) -> AocResult {
    let constellations = read_constellations(data)?;
    answer(constellations.len())
}

fn read_constellations(data: &AocData) -> Result<Vec<Vec<Vector4>>> {
    let mut constellations: Vec<Vec<Vector4>> = Vec::new();
    let mut groups: Vec<usize> = Vec::new();

    for line in data.lines()? {
        let v: Vector4 = line.parse()?;
        for (index, constellation) in constellations.iter_mut().enumerate() {
            if constellation.iter().any(|c| v.dist(c) <= 3) {
                if groups.is_empty() {
                    constellation.push(v);
                }
                groups.push(index);
            }
        }

        if groups.len() > 1 {
            let target = groups[0];
            for &index in groups[1..].iter().rev() {
                let to_join = constellations.remove(index);
                constellations[target].extend(to_join.into_iter());
            }
        } else if groups.is_empty() {
            constellations.push(vec![v]);
        }
        groups.clear();
    }

    Ok(constellations)
}

#[derive(Debug, Copy, Clone)]
struct Vector4 {
    x: i32,
    y: i32,
    z: i32,
    w: i32,
}

impl Vector4 {
    fn dist(&self, o: &Vector4) -> u32 {
        (self.x - o.x).abs() as u32
            + (self.y - o.y).abs() as u32
            + (self.z - o.z).abs() as u32
            + (self.w - o.w).abs() as u32
    }
}

impl FromStr for Vector4 {
    type Err = Error;
    fn from_str(s: &str) -> Result<Vector4> {
        let mut i = s.split(",");
        let x: i32 = i
            .next()
            .ok_or_else(|| format_err!("x component missing"))?
            .trim()
            .parse()?;
        let y: i32 = i
            .next()
            .ok_or_else(|| format_err!("y component missing"))?
            .trim()
            .parse()?;
        let z: i32 = i
            .next()
            .ok_or_else(|| format_err!("z component missing"))?
            .trim()
            .parse()?;
        let w: i32 = i
            .next()
            .ok_or_else(|| format_err!("w component missing"))?
            .trim()
            .parse()?;
        if i.next().is_some() {
            bail!("more than 4 components in vec4");
        }
        Ok(Vector4 { x, y, z, w })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn count_constellations(s: &'static str) -> usize {
        read_constellations(&AocData::from_str(s)).unwrap().len()
    }

    #[test]
    fn part1() {
        assert_eq!(
            count_constellations(
                "
        0,0,0,0
        3,0,0,0
        0,3,0,0
        0,0,3,0
        0,0,0,3
        0,0,0,6
        9,0,0,0
       12,0,0,0
               "
            ),
            2
        );

        assert_eq!(
            count_constellations(
                "
        -1,2,2,0
        0,0,2,-2
        0,0,0,-2
        -1,2,0,0
        -2,-2,-2,2
        3,0,2,-1
        -1,3,2,2
        -1,0,-1,0
        0,2,1,-2
        3,0,0,0
        "
            ),
            4
        );
        assert_eq!(
            count_constellations(
                "
        1,-1,0,1
        2,0,-1,0
        3,2,-1,0
        0,0,3,1
        0,0,-1,-1
        2,3,-2,0
        -2,2,0,0
        2,-2,0,-1
        1,-1,0,-1
        3,2,0,2
                "
            ),
            3
        );
        assert_eq!(
            count_constellations(
                "
        1,-1,-1,-2
        -2,-2,0,1
        0,2,1,3
        -2,3,-2,1
        0,2,3,-2
        -1,-1,1,-2
        0,-2,-1,0
        -2,2,3,-1
        1,2,2,0
        -1,-2,0,-2
                "
            ),
            8
        );
    }
}
