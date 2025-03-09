use super::{animation_config::AnimationConfig, atlas_config::AtlasConfig};

pub fn forward_animation(frame_advance: usize, atlas_config: &mut AtlasConfig, animation_config: &AnimationConfig) -> usize {
    if atlas_config.current_frame < animation_config.frame_range.start {
        atlas_config.current_frame = animation_config.frame_range.start;
        return atlas_config.current_frame;
    }
    
    let new_frame = atlas_config.current_frame + frame_advance;
    
    if animation_config.looping {
        return if new_frame >= animation_config.frame_range.end {
            animation_config.frame_range.start + (new_frame - animation_config.frame_range.start) % (animation_config.frame_range.end - animation_config.frame_range.start)
        } else {
            new_frame
        };
    } else {
        if new_frame >= animation_config.frame_range.end {
            return animation_config.frame_range.end - 1;
        } else {
            return new_frame;
        }
    }
}


pub fn backward_animation(frame_advance: usize, atlas_config: &mut AtlasConfig, animation_config: &AnimationConfig) -> usize {
    if atlas_config.current_frame > animation_config.frame_range.end {
        atlas_config.current_frame = animation_config.frame_range.end;
        return atlas_config.current_frame;
    }

    let new_frame = if atlas_config.current_frame >= frame_advance {
        atlas_config.current_frame - frame_advance
    } else {
        animation_config.frame_range.end - (frame_advance - atlas_config.current_frame)
    };

    if animation_config.looping {
        return if new_frame < animation_config.frame_range.start {
            animation_config.frame_range.end - (animation_config.frame_range.start - new_frame) % (animation_config.frame_range.end - animation_config.frame_range.start)
        } else {
            new_frame
        };
    } else {
        if new_frame < animation_config.frame_range.start {
            return animation_config.frame_range.start;
        } else {
            return new_frame;
        }
    }
}

pub fn random_animation(animation_config: &AnimationConfig) -> usize {
    use rand::Rng;
    let mut rng = rand::rng();
    rng.random_range(animation_config.frame_range.start..animation_config.frame_range.end)
}
