export type IconProps = React.SVGProps<SVGSVGElement> & { size?: number }

export function makeIcon(
    Svg: React.FC<React.SVGProps<SVGSVGElement>>,
    displayName: string,
) {
    function Icon({ size = 16, ...props }: IconProps) {
        return (
            <Svg
                width={size}
                height={size}
                aria-hidden
                focusable={false}
                {...props}
            />
        )
    }
    Icon.displayName = displayName
    return Icon
}
