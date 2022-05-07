import glob
import sys
import os


def stitch(file_patturn):
    """
    Stich together the individual render tiles at a given patturn.
    """

    tiles = glob.glob(f"{file_patturn}*")
    width = len(glob.glob(f"{file_patturn}_0_*"))
    height = int(len(tiles) / width)

    for n in range(height):
        slice_patturn = f"{file_patturn}_{n}_*"
        os.system(f"convert -append {slice_patturn} {file_patturn}_slice_{n}.png")

    os.system(f"convert +append {file_patturn}_slice_* {file_patturn}.png")


if __name__ == "__main__":
    input_dir = sys.argv[1]
    for sub_dir in [
        f
        for f in os.listdir(input_dir)
        if not os.path.isfile(os.path.join(input_dir, f))
    ]:
        stitch(os.path.join(input_dir, sub_dir, "colour"))
