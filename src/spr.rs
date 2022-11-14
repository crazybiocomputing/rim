//
//  RIM - Rust Image
//  Copyright (C) 2022  Jean-Christophe Taveau
//  This file is part of RIM
//
// This program is free software: you can redistribute it and/or modify it
// under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.
//
//  You should have received a copy of the GNU General Public License
//  along with RIM.  If not, see <http://www.gnu.org/licenses/>.
//
// Group : SPR in real space / frequency space (fftw3)
// Authors : Léonard Brindel, Océane Dorémus, Léo Gillet
//


fn pub image_projection(ImageStack ist) -> Vec{
   let img_projection_stack = Vec::new();
   for _ in 10 {
        img_projection_stack.push(ist);
   }
   return img_projection_stack;
}

fn pub image_stack_projection(ImageStacks ist_stack) -> Vec{
    let all_img_projection = Vec::new();
    for img_stack in ist_stack{
        all_img_projection.push(image_projection(img_stack);
    }
    return all_img_projection;
}