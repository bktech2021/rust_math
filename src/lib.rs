//TODO: Binary search and some sorting functions needed to be added
pub mod numbers {
    pub fn pi() -> f32 {
        return 3.1415926535897932384626433832795028841971693993751058209749445923078164062862089986280348253421170679;
    }
}

#[allow(dead_code)]
pub mod list {
    //This function is used to sort the list. (Bubble sort)
    pub fn bubble_sort(list: &mut [usize]) {
        for _ in 0..list.len() {
            for j in 0..(&list.len() - 1) {
                if list[j] > list[j + 1] {
                    list.swap(j, j + 1);
                }
            }
        }
    }

    pub fn reverse_bsort(list: &mut [usize]) {
        for _ in 0..list.len() {
            for j in 0..(&list.len() - 1) {
                if list[j] < list[j + 1] {
                    list.swap(j, j + 1);
                }
            }
        }
    }

    //THIS FUNCTION NEEDS SORTED ARRAY
    pub fn med(list: &[usize]) -> i32 {
        let len = list.len();
        if len % 2 == 0 {
            let med1 = list[len / 2];
            let med2 = list[len / 2 - 1];
            let med = (med1 + med2) / 2;
            return med as i32;
        } else {
            let med = list[(len - 1) / 2];
            return med as i32;
        }
    }

    //This function works whether the list is sorted or not, if sorted, returns med of the list, else -1. Uses extra resource. (May be a problem for old systems)
    pub fn safe_med(list: &[usize]) -> i32 {
        if check_sorted(list) {
            return med(list);
        } else {
            return -1;
        }
    }

    //This function returns the average of the list.
    pub fn avg(list: &[usize]) -> f32 {
        let mut sum = 0;
        for i in list {
            sum += i;
        }
        let avg = sum as f32 / list.len() as f32;
        return avg;
    }

    //This function returns the mode of the list.
    pub fn mode(list: &[usize]) -> i32 {
        let mut mode = 0;
        let mut max = 0;
        for i in list {
            let mut count = 0;
            for j in list {
                if i == j {
                    count += 1;
                }
            }
            if count > max {
                max = count;
                mode = *i;
            }
        }
        return mode as i32;
    }

    //This function returns the standard deviation of the list.
    pub fn std_deviation(list: &[usize]) -> f32 {
        let avg = avg(list);
        let mut sum: f32 = 0.00;
        let len = list.len() as f32 - 1.00;
        for i in list {
            sum += super::num::abs((*i as f32 - avg).powf(2.00));
        }
        let std_dev = (sum / len).sqrt();
        return std_dev;
    }

    //This function returns the highest number in the list.
    pub fn highest(list: &[usize]) -> i32 {
        let mut peak = list[0];
        for i in list {
            if *i > peak {
                peak = *i;
            }
        }
        return peak as i32;
    }

    //This function returns the lowest number in the list.
    pub fn lowest(list: &[usize]) -> i32 {
        let mut low = list[0];
        for i in list {
            if *i < low {
                low = *i;
            }
        }
        return low as i32;
    }

    //This function returns the true if the list is forted, else false.
    pub fn check_sorted(list: &[usize]) -> bool {
        for i in 0..list.len() - 1 {
            if list[i] > list[i + 1] {
                return false;
            }
        }
        return true;
    }
}

#[allow(dead_code)]
pub mod num {
    pub fn abs(num: f32) -> f32 {
        if num < 0.0 {
            return -num;
        } else {
            return num;
        }
    }

    pub fn sqrt(num: f32) -> f32 {
        let mut number = num;
        while abs(num - number * number) > 0.00001 {
            number = (number + num / number) / 2.00;
        }
        return number;
    }
    //This function returns the factorial of the number.
    pub fn factorial(num: i128) -> i128 {
        if num == 0 {
            return 1;
        } else {
            return num * factorial(num - 1);
        }
    }

    //This function checks if number prime or not
    pub fn is_prime(number: i32) -> bool {
        /*
        Using 6k + 1 optimization: all numbers are expresible as 6k + i with i = {-1, 0, 1, 2, 3, 4}.
        Test if n is divisible by 2 or 3, with that you remove: 6k, 6k + 2, 6k + 4, 6k + 3.
        This means that you only have to test numbers of the form 6k - 1 and 6k + 1 less than sqrt(n)
        These numbers are 5, 11, 17, 23, 29, 35... and 7, 13, 19, 25, 31, 37...
        */
        if number == 2 || number == 3 {
            return true;
        }
        if number % 2 == 0 || number % 3 == 0 {
            return false;
        }

        for i in (1..).map(|k| 6 * k - 1).take_while(|m| m * m <= number) {
            if number % i == 0 || number % (i + 2) == 0 {
                return false;
            }
        }

        return true;
    }

    //This function finds greatest common divisor of two numbers
    pub fn gcd(a: i32, b: i32) -> i32 {
        if a == 0 {
            return b;
        } else {
            return gcd(b % a, a);
        }
    }

    //This function finds least common multiple of two numbers
    pub fn lcm(a: i32, b: i32) -> i32 {
        return a * b / gcd(a, b);
    }

    //This is for finding prime factors
    pub fn prime_factor(int32: i32) -> Vec<i32> {
        let mut c = 2;
        let mut iint32 = int32;
        let mut ret: Vec<i32> = Vec::new();
        while iint32 > 1 {
            if iint32 % c == 0 {
                ret.push(c);
                iint32 /= c;
            } else {
                c += 1;
            }
        }
        return ret;
    }
}

#[allow(dead_code)]
pub mod trigonometry {
    use super::numbers::pi;
    //This function converts a number from degrees to radians.
    pub fn deg2rad(x: f32) -> f32 {
        return x * pi() / 180.00;
    }

    //This function finds quadrant of angle.
    pub fn find_quadrant(angle: f32) -> i32 {
        if angle >= 360.00 {
            return find_quadrant(angle - 360.00);
        } else if angle <= 0.00 {
            return find_quadrant(angle + 360.00);
        } else {
            return ((angle / 90.00) + 1.00) as i32;
        }
    }

    //This function convert the number from radians to degrees.
    pub fn rad2deg(x: f32) -> f32 {
        return x * 180.00 / pi();
    }

    //This function finds the sine of the angle.
    pub fn sin(ang: f32) -> f32 {
        //Algorithm for calculating sine
        //1. Use periodicity
        //If angle is between 0 and 360, no problem, but if not, reduce x so that it lies in the range 0≤x≤360° by adding or subtracting a suitable multiple of 360° from it (we are assuming that angle x is measured in degrees).

        let mut angle = ang;
        while angle < 0.0 {
            angle += 360.0;
        }
        while angle > 360.0 {
            angle -= 360.0;
        }

        //2. Using symetry
        //If angle is quadrant 1, go to step 3.
        let quadrant = find_quadrant(angle);
        if quadrant == 2 {
            angle = 180.0 - angle;
        } else if quadrant == 3 {
            angle = 180.0 - angle;
        } else if quadrant == 4 {
            angle = angle - 360.0;
        }

        //3. Using the cofunction
        if angle >= 45.0 {
            return cos(90.0 - angle);
        } else if angle < 45.0 {
            //4. Using the sine polynomial
            angle = deg2rad(angle);
            return angle - (angle.powf(3.0) / 6.0) + (angle.powf(5.0) / 120.0);
        }
        return -1.0;
    }

    pub fn cos(ang: f32) -> f32 {
        //1. Use periodicity
        let mut angle = ang;
        while angle < 0.0 {
            angle += 360.0;
        }
        while angle > 360.0 {
            angle -= 360.0;
        }
        //2. Use symetry
        let quadrant = find_quadrant(angle);
        if quadrant == 1 {
            //3. Using cofunction
            if angle >= 45.0 {
                return sin(90.0 - angle);
            } else if angle < 45.0 {
                angle = deg2rad(angle);
                //4. Using cosine polynomial
                return 1.0 - (angle.powf(2.0) / 2.0) + (angle.powf(4.0) / 24.0)
                    - (angle.powf(6.0) / 720.0);
            }
        } else if quadrant == 2 {
            angle = 180.0 - angle;
            return -1.0 * cos(angle);
        } else if quadrant == 3 {
            angle = angle - 180.0;
            return -1.0 * cos(angle);
        } else if quadrant == 4 {
            angle = 360.0 - angle;
            return cos(angle);
        }
        return -1.0;
    }
    pub fn tan(ang: f32) -> f32 {
        let mut angle = ang;

        //1. Reduce angle between 180 and 0
        while angle > 180.0 {
            angle -= 180.0;
        }
        while angle < 0.0 {
            angle += 180.0;
        }

        //2. Reduce angle between 90 and 0
        let quadrant = find_quadrant(angle);
        if quadrant == 2 {
            return -1.0 * tan(180.0 - angle);
        } else if quadrant == 1 {
            if angle >= 45.0 {
                //3. Cofunction
                return 1.0 / tan(90.0 - angle);
            } else if angle < 45.0 {
                if angle >= 22.5 {
                    return (2.0 * tan(angle / 2.0)) / (1.0 - tan(angle / 2.0).powf(2.0));
                } else if angle < 22.5 {
                    angle = deg2rad(angle);
                    return angle
                        + (angle.powf(3.0) / 3.0)
                        + (2.0 * angle.powf(5.0) / 15.0)
                        + (17.0 * angle.powf(7.0) / 315.0);
                }
            }
        }
        return -1.0;
    }
    pub fn cotan(ang: f32) -> f32 {
        return 1.0 / tan(ang);
    }
    pub fn sec(ang: f32) -> f32 {
        return 1.0 / cos(ang);
    }
    pub fn cosec(ang: f32) -> f32 {
        return 1.0 / sin(ang);
    }
    pub fn arctan(x: f32) -> f32 {
        let y = x;
        if y < 0.0 {
            return -1.0 * arctan(-1.0 * y);
        } else
        /* if positive */
        {
            if y < 1.0 {
                if y <= 0.267949 {
                    return y - (y.powf(3.0) / 3.0) + (y.powf(5.0) / 5.0);
                } else {
                    return rad2deg(
                        (super::numbers::pi() / 6.0)
                            + arctan((super::num::sqrt(3.0) * y) - 1.0)
                                / arctan(super::num::sqrt(3.0) + y),
                    );
                }
            } else {
                return (super::numbers::pi() / 2.0) - arctan(1.0 / y);
            }
        }
    }
    pub fn arcsin(x: f32) -> f32 {
        return arctan(x / super::num::sqrt(1.0 - x.powf(2.0)));
    }
    pub fn arccos(x: f32) -> f32 {
        return arctan(super::num::sqrt(1.0 - x.powf(2.0)) / x);
    }
}
