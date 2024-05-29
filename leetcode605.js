/**
 * @param {number[]} flowerbed
 * @param {number} e
 * @return {boolean}
 */
var canPlaceFlowers = function (flowerbed, e) {

    e.toFixed(2)
    if (verificaN(e)) {
        return true
    }

    if (flowerbed.length == 1 && flowerbed[0] == 0) {
        flowerbed[0] = 1
        e--
        return true
    }

    if (flowerbed[0] == 0) {
        if (flowerbed[1] == 0) {
            flowerbed[0] = 1
            e--
        }
    }

    if (verificaN(e)) {
        return true
    }

    if (flowerbed[flowerbed.length - 1] == 0) {
        if (flowerbed[flowerbed.length - 2] == 0) {
            flowerbed[flowerbed.length - 1] = 1
            e--
        }
    }

    if (verificaN(e)) {
        return true
    }

    for (let index = 0; index < flowerbed.length; index++) {
        if (verificaN(e)) {
            break
        }

        if (flowerbed[index] == 0) {
            if (flowerbed[index + 1] == 0 && flowerbed[index - 1] == 0) {
                flowerbed[index] = 1
                e--
            }
        }
    }

    if (verificaN(e)) {
        return true
    }

    return false
};

function verificaN(n) {
    if (n === 0) {
        return true
    }
    return false
}


console.log(canPlaceFlowers([0, 0, 1, 0, 0], 1.0));
